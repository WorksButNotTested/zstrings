# ZStrings
`zstrings` is similar to the strings utility, but is intended to allow searching for NULL or zero terminated strings.

# Example
The following example prints the zero terminated strings found in the `ls` binary. It precedes each entry with the filename, shows the offset in hexadecimal and shows only strings which are at least 10 characters long:
```bash
$ zstrings -f -t x -n 10 /bin/ls
/bin/ls: 0x00000318 /lib64/ld-linux-x86-64.so.2
/bin/ls: 0x00001191 libselinux.so.1
/bin/ls: 0x000011a1 _ITM_deregisterTMCloneTable
/bin/ls: 0x000011bd __gmon_start__
/bin/ls: 0x000011cc _ITM_registerTMCloneTable
/bin/ls: 0x000011e6 fgetfilecon
/bin/ls: 0x000011fa lgetfilecon
/bin/ls: 0x00001227 __printf_chk
/bin/ls: 0x0000126d fflush_unlocked
/bin/ls: 0x000012c0 sigprocmask
...
```
# Usage
```bash
$ zstrings --help
Tool for zero terminated strings in binary files

Usage: zstrings [OPTIONS] <INPUT>

Arguments:
  <INPUT>
          Input file

Options:
  -l, --log-level <LOG_LEVEL>
          Log level

          [default: info]
          [possible values: off, error, warn, info, debug, trace]

  -n, --min-len <LENGTH>
          String Length

          [default: 4]

  -t, --radix <RADIX>
          Offset Radix

          [possible values: d, x]

  -f, --print-file-name
          File Name

  -a, --alignment <ALIGNMENT>
          Alignment

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
