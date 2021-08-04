#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!("crono-bindgen/crono_hptdc.rs");
include!("crono-bindgen/crono_tt4.rs");


// for tt4:
use std::{ffi::CStr, ffi::CString, os::raw::c_char, mem::MaybeUninit, time::{Duration, Instant}};

// never use String.as_ptr as const *u8 to get a C string (but it is fine for &str)!
fn c_string_of<T: Into<Vec<u8>>>(s: T) -> *mut c_char {
    return CString::new(s).unwrap().into_raw();
}


// readtest routine using manager.Read 
// ### test signals:
// ch0: 0.1MHz "trigger" SEEMS not to be needed!
// ch1,2: 0.5MHz, ch2 is 40ns later than ch1. 5 MHz = 200ns
#[allow(unused_assignments)]
fn readtest_hptdc() {
    let mut lasttime = Instant::now();
    let mut m = unsafe { TDCManager::new(0x1A13, 0x0001) };
    unsafe { 
        m.Init(); 
        // if !manager.ReadConfigFile(c_string_of("hptdc.cfg")) { panic!("read config failed!"); }
        // unclear if GroupRangeEnd has correct effect, also shorter than trigger period all hits are recorded...
        if !m.ReadConfigString(c_string_of("GroupRangeStart 0ns")) { panic!("read config string failed!"); }
        if !m.ReadConfigString(c_string_of("GroupRangeEnd 20us")) { panic!("read config string failed!"); }
        if !m.ReadConfigString(c_string_of("RisingEnable none")) { panic!("read config string failed!"); }
        if !m.ReadConfigString(c_string_of("GroupingEnable false")) { panic!("read config string failed!"); }
        if !m.ReadConfigString(c_string_of("TriggerDeadTime 0s")) { panic!("read config string failed!"); }
        if !m.ReadConfigString(c_string_of("VHR false")) { panic!("read config string failed!"); }
        if !m.ReadConfigString(c_string_of("BufferSize 16")) { panic!("read config string failed!"); }
        if !m.ReadConfigString(c_string_of("FallingEnable none")) { panic!("read config string test failed!"); }
        if !m.ReadConfigString(c_string_of("RisingEnable 0-3")) { panic!("read config string test failed!"); }
        
        let binsize = 25i64; // 25ps
        m.Reconfigure();

        m.Start();
        let mut rollover = 0i64; // rollover time
        let mut lastch1 = 0i64;
        let mut lastch2 = 0i64;
        let mut dtsame = 0i64;
        let mut dtother = 0i64;
        let mut total = 0;
        let mut totch1 = 0i64;
        let mut totch2 = 0i64;
        println!("before loop: setup took {:?}", lasttime.elapsed());
        lasttime = Instant::now();
        let printdebug = false;
        while total < 10000000 { // disable println for higher numbers!
            let mut buffer: [HIT; 2000] = [0; 2000usize];
            let count = m.Read(buffer.as_mut_ptr(),2000); 
            for i in 0..count as usize {
                let b3130 = buffer[i] >> 30;
                if b3130 > 0 { // falling, raising, error
                    let channel = buffer[i] >> 24 & 0b111111;
                    if b3130 == 1 {
                        println!("hit error ch={} buf={:#034b}", channel, buffer[i]);
                    } else {
                        let time: i64 = (rollover + (buffer[i] & 0xffffff) as i64) * binsize; // 25 ps per bin
                        // note: using Read, the hits are NOT time-ordered!
                        if channel == 1 { dtsame=time-lastch1 ; dtother=time-lastch2 ; lastch1=time ; totch1+=1 } else 
                        if channel == 2 { dtsame=time-lastch2 ; dtother=time-lastch1 ; lastch2=time ; totch2+=1  } else { dtother=0 ; dtsame=0 }
                        if printdebug { println!("hit raising/falling: b3130={} ch={} t={} dtsame={} dtother={} buf={:#034b}", b3130, channel, time, dtsame, dtother, buffer[i]); }
                    }
                } else {
                    if buffer[i] >> 28 == 0 {
                        println!("hit group t={} buf={:#034b}", buffer[i] & 0xffffff, buffer[i]); // The time values of the following hits should be added to the trigger time to obtain the absolute time of the transition
                    } else if buffer[i] >> 24 == 0b00010000 {
                        if printdebug { println!("hit rollover t={} buf={:#034b}", buffer[i] & 0xffffff, buffer[i]); }
                        rollover = ((buffer[i] & 0xffffff) as i64) << 24;
                    } else if buffer[i] >> 27 == 0b00011 {
                        println!("hit levelinfo buf={:#034b}", buffer[i]);
                    } else if buffer[i] >> 24 == 0b00100000 {
                        println!("hit resolution binsize={} buf={:#034b}", buffer[i] & 0xffffff, buffer[i]);
                    } else {
                        println!("hit unknown: b3130={:#04b}  buf={:#034b}:", b3130, buffer[i]);
                    }
                }
                let now = Instant::now();
                if now-lasttime > Duration::new(1,0) {
                    println!("dt= {:?} cnt1={} cnt2={}", now-lasttime, totch1, totch2);
                    totch1=0;totch2=0;
                    lasttime = now;
                }
            }
            total += count;
        }
        m.Stop();
        m.CleanUp();
    }
}

pub fn readtest_tt4() {

    let mut error_code: i32 = -99;
    let error_message: CString = CString::new([1u8; 256]).unwrap();
    let res = unsafe { timetagger4_count_devices(&mut error_code, &mut error_message.as_ptr()) };
    println!("timetagger4_count_devices: res={} ec={}", res, error_code);
    if res != 1 {
        panic!("wrong number of devices {}", res);
    }
    if error_code != 0 {
        println!("  error message: {:?}", error_message);
    }

    // init

    // https://stackoverflow.com/questions/43350167/how-do-i-initialize-an-opaque-c-struct-when-using-rust-ffi
    let mut res: i32;
    let mut params_unsafe: MaybeUninit<timetagger4_init_parameters> = MaybeUninit::uninit();
    res = unsafe { timetagger4_get_default_init_parameters(params_unsafe.as_mut_ptr()) };
    let mut params: timetagger4_init_parameters = unsafe { params_unsafe.assume_init() };
    println!("res={} em={:?}", res, &params);
    params.buffer_size[0] = 100*1024*1024; // set to 100 MB, only first entry used.

    let mut device: timetagger4_device = unsafe { *timetagger4_init(&mut params, &mut error_code, &mut error_message.as_ptr()).as_ref().unwrap() };
    println!("YYYinit: ec={} res={:?}", error_code, device);
    if error_code != 0 {
        println!("  error message: {:?}", error_message);
    }

    // configuration
    let mut config_unsafe: MaybeUninit<timetagger4_configuration> = MaybeUninit::uninit();
    res = unsafe { timetagger4_get_default_configuration(&mut device, config_unsafe.as_mut_ptr()) };
    if res != TIMETAGGER4_OK as i32 { panic!("get default config failed!"); }
    let mut config: timetagger4_configuration = unsafe { config_unsafe.assume_init() };

    for i in 0..TIMETAGGER4_TDC_CHANNEL_COUNT as usize {
        config.channel[i].enabled = 1;
        config.trigger[i + 1].falling = 0;
        config.trigger[i + 1].rising = 1;
        config.dc_offset[i + 1] = 0.5;
    }
    // sync channel
    config.dc_offset[0] = 0.5;
    config.trigger[0].falling = 0;  
    config.trigger[0].rising = 1;

    // write back
    res = unsafe { timetagger4_configure(&mut device, &mut config) };
    if res != TIMETAGGER4_OK as i32 {
        panic!("write configuration failed!");
    }

    // this is a #define in crono_interface.h
    // #define crono_next_packet(current) ((crono_packet*) (((__int64) (current)) +( ((current)->type&128?0:(current)->length) + 2) * 8))
    fn crono_next_packet(current: *mut crono_packet) -> *mut crono_packet {
        let res = unsafe { (current as i64) + (((if ((*current).type_ & 128) != 0 { 128 } else { (*current).length as i64 }) + 2) * 8 ) };
        return res as *mut crono_packet
    }

    let mut read_config = timetagger4_read_in {
        acknowledge_last_read: 1,
    };

    // flush buffers
    let mut read_data_unsafeX: MaybeUninit<timetagger4_read_out> = MaybeUninit::uninit();
    while unsafe { timetagger4_read(&mut device, &mut read_config, read_data_unsafeX.as_mut_ptr()) } == 0 {
        println!("flush buffer: did read!");
    }

    // do a test read!
    let res = unsafe { timetagger4_start_capture(&mut device) };
    if res != CRONO_OK as i32 {
        panic!("could not start capturing!");
    }
    let mut i = 0;
     { //no loop, test how much stuff comes in one read.
        println!("readtest: loop begin!");
        let mut read_data_unsafe: MaybeUninit<timetagger4_read_out> = MaybeUninit::uninit();
        let res = unsafe { timetagger4_read(&mut device, &mut read_config, read_data_unsafe.as_mut_ptr()) };
        let read_data: timetagger4_read_out = unsafe { read_data_unsafe.assume_init() };
        let s = unsafe { CStr::from_ptr(read_data.error_message) };
        i += 1;

        println!("read i={} res={} readdata={:?} em={:?}", i, res, read_data, s);

        let mut p = read_data.first_packet;
        while p != read_data.last_packet {
            let numhits = unsafe { 
                let x = 2 * (*p).length;
                if (*p).flags & 0x1u8 > 0 { x - 1 } else { x } // flag determines if one hit less in memory
            };
            let _packet_timestamp = unsafe { (*p).timestamp };
            unsafe { println!(" packet: p={:?} addr={}", *p, (p as i64)) };
                for i in 0..numhits as u64 {
                unsafe { 
                    let h = *(((*p).data.as_ptr() as u64 + 4 * i) as *const u32); // 4 bytes per hit!
                    let hitt = h >> 8u32 & 0xffffffu32;
                    let chan = h & 0xfu32;
                    // b7,b6 must be 0,1 !
                    println!("  hit {}: {:x} b7={} b6={} ch={} hitt={}", i, h, (h >> 7) & 1, (h >> 6) & 1, chan, hitt);
                };
            }
            p = crono_next_packet(p);
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}




fn main() {

    readtest_hptdc();

//    readtest_tt4();

}
