### cronologicrust-simple (for windows 10)
this reads hits from the timetagger cards HPTDC (PCI) and TT4 (PCIe) from cronologic.de 
and evaluates them. It is written in rust (a safe successor of C) and
contains wrappers for many functions of those cards (thanks to rust-bindgen, it works for the C drivers for tt4 and also for the old c++ drivers for hptdc). 
We use it for multi-photon quantum optics experiments with single-photon detectors.
It runs a TCP server for communication with a control program.
Have unlimited flexibility and save several 10k compared to other solutions.

If this is useful, please tell us: https://quphotonics.org or better buy us a card!

### setup development:
* setup rust normally via 64bit rustup, use vscode with rust-analyze plugin to develop.
* the crono_* folders contain the headers and libs for x64 versions of hptdc, tt4. 
* src/crono_* are the generated rust files, included in git so they don't have to be re-build all the time. Also: IDE support...

### rust-bindgen
only needed if something changed in API! See build.rs, set "genbindings", do before:

  * https://github.com/rust-lang/rust-bindgen
  * cargo install bindgen
  * https://rust-lang.github.io/rust-bindgen/requirements.html
  * download & install clang llvm winx64 (e.g. LLVM-12.0.0-win64.exe), select "add to path", C:\Program Files\LLVM
	* modify because of https://github.com/rust-lang/rust-bindgen/issues/1252:
```
struct timetagger4_read_in {	
		crono_bool_t acknowledge_last_read; //!< timetagger4_read automatically acknowledges packets from the last read
	};

-->

typedef struct {	
		crono_bool_t acknowledge_last_read; //!< timetagger4_read automatically acknowledges packets from the last read
	} timetagger4_read_in ;
```    

### scripts

* start-simple.bat : a simple thing to test the cards.

### important notes:
* https://github.com/cronologic-de
 
