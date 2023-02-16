use {
    anyhow::{anyhow, Result},
    indicatif::{ParallelProgressIterator, ProgressBar},
    rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
    regex::bytes::Regex,
    std::{
        collections::{hash_map::Iter, HashMap},
        ffi::CStr,
        fmt,
    },
};

type ZStringsMatches = HashMap<usize, String>;

#[derive(Debug, Clone)]
pub struct ZStringResult(usize, String);

impl ZStringResult {
    pub fn offset(&self) -> usize {
        self.0
    }

    pub fn string(&self) -> String {
        self.1.clone()
    }

    pub fn overlaps(&self, other: &ZStringResult) -> bool {
        let s1 = self.0;
        let e1 = s1 + self.1.len();
        let s2 = other.0;
        let e2 = s2 + other.1.len();

        if e1 < s2 {
            return false;
        }
        if s1 > e2 {
            return false;
        }
        true
    }
}

type ZStringResults = Vec<ZStringResult>;

#[derive(Default, Clone)]
pub struct ZStrings(ZStringsMatches);

impl<'a> IntoIterator for &'a ZStrings {
    type Item = (&'a usize, &'a String);
    type IntoIter = Iter<'a, usize, String>;
    fn into_iter(self) -> Iter<'a, usize, String> {
        self.0.iter()
    }
}

const MAX_STRING_LEN: usize = 1024;

impl ZStrings {
    pub fn new_parallel(
        data: &[u8],
        length: usize,
        num_chunks: usize,
        progress_bar: ProgressBar,
    ) -> Result<ZStrings> {
        let len = data.len();
        let chunk_len = usize::max(MAX_STRING_LEN, len / num_chunks);

        let chunks = (0..len)
            .step_by(chunk_len)
            .map(|x| {
                let limit = usize::min(x + chunk_len + MAX_STRING_LEN, len);
                data.get(x..limit).map(|d| (x, d))
            })
            .collect::<Option<Vec<(usize, &[u8])>>>()
            .ok_or_else(|| anyhow!("Failed to read chunks"))?;

        progress_bar.set_length(chunks.len() as u64);

        let results = chunks
            .into_par_iter()
            .progress_with(progress_bar)
            .map(|(i, ck)| ZStrings::new(i, ck, length))
            .collect::<Result<Vec<ZStrings>>>()?;

        let reduced = results
            .par_iter()
            .cloned()
            .reduce(ZStrings::default, |a, b| {
                let mut results: ZStringsMatches = a.0.clone();
                results.extend(b.0);
                ZStrings(results)
            });
        Ok(reduced)
    }

    pub fn new(offset: usize, chunk: &[u8], length: usize) -> Result<ZStrings> {
        let regex_string = format!(r"(?-u)[\x09\x0a\x0d\x20-\x7e]{{{},}}\x00", length);
        let regex = Regex::new(&regex_string).unwrap();
        let results = regex
            .find_iter(chunk)
            .map(|x| {
                let string = CStr::from_bytes_with_nul(x.as_bytes())
                    .map_err(|e| anyhow!("Failed to create CString: {e:}"))?
                    .to_str()
                    .map_err(|e| anyhow!("Failed to convert CString: {e:}"));
                match string {
                    Ok(s) => Ok((offset + x.start(), s.to_string())),
                    Err(e) => Err(e),
                }
            })
            .collect::<Result<ZStringsMatches>>()?;
        Ok(ZStrings(results))
    }

    pub fn results(&self, alignment: Option<usize>) -> ZStringResults {
        let mut results = self
            .into_iter()
            .map(|(x, y)| ZStringResult(x.clone(), y.clone()))
            .collect::<ZStringResults>();
        results.sort_by_key(|x| x.0);
        let mut distinct = ZStringResults::default();
        for result in results {
            if distinct.len() == 0 {
                distinct.push(result);
                continue;
            }

            let last = &distinct[distinct.len() - 1];
            if !result.overlaps(last) {
                distinct.push(result);
            }
        }
        let trimmed = distinct
            .iter()
            .map(|r| {
                let trim = r
                    .string()
                    .chars()
                    .filter(|c| match c {
                        '\x07' => false,
                        '\x0a' => false,
                        '\x0d' => false,
                        _ => true,
                    })
                    .collect::<String>();
                ZStringResult(r.offset(), trim)
            })
            .collect::<ZStringResults>();

        if let Some(a) = alignment {
            trimmed.iter().filter(|r| r.offset() & (a-1) == 0).cloned().collect::<ZStringResults>()
        } else {
            trimmed
        }
    }
}

impl fmt::Debug for ZStrings {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for r in self.results(None) {
            writeln!(fmt, "{r:#?}")?;
        }
        Ok(())
    }
}
