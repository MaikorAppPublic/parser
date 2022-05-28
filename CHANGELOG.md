## Pre-alpha

### 0.1.9
- Update dependencies
  - Platform to 0.1.25
- Fix bug with whitespace handling

### 0.1.8
- Update dependencies
  - Platform to 0.1.24
- Add `BMUL`

### 0.1.7
- Update dependencies
  - Platform to 0.1.23
- Add `RCR` and `RCL`

### 0.1.6
- Update dependencies
  - Platform to 0.1.22
- Add `MSWP`

### 0.1.5
- Improve error data
- Fix parsing issues

### 0.1.4
- Make `Program` fields public

### 0.1.3
- Rewrite to support index addressing

### 0.1.2

- *BREAKING CHANGE*
- Fix MUL and MULS parsing
- Update dependencies
  - Language to 0.1.11 

### 0.1.1

- Fix issue where size indicator (.B and .W) had to be capitalised)
- Change register byte to use id instead of offset
  - Update language dep 
  - As language registers should be independent of VM implementation

### 0.1.0

- Initial release
- Very basic parser implementation