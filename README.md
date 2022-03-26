# Win32 Fuzzer

This repository contains fuzzer to test Windows API functions(Win32 API).

I created it to help Wine to better mimic Windows OS and allow running more Windows software on Linux. 

I think that creating also 32 bit binaries, will also help to test ReactOS, but for now there are a lot of problems with compiling it. 

Looks that currently tests - 7322 functions from all 10540 parsed from `windows-rs` source.  
When looking Wine source code, executing `rg stub -g *.spec` shows that Wine recognizes that there is ~25000 unimplemented functions and around ~26000 implemented by executing `rg stub -g *.spec`.

Currently, supported version of [windows-rs](https://github.com/microsoft/windows-rs) is  `0.33.0`.

## How to use it
- Install wine(7.4 works fine, any lower version may cause crashes, any newer may also crash, but this is bug in Wine, not in this project)
- Install GNU Rust inside Wine(I used 1.59.0, but any higher version should work fine) - https://forge.rust-lang.org/infra/other-installation-methods.html#standalone-installers
- Clone/Download `windows-rs` repository(required to get info about windows api functions) - `git clone https://github.com/microsoft/windows-rs.git`
- Clone this repository - `git clone https://github.com/qarmin/Win32Fuzzer.git`
- Compile and run project with 1 argument set as path to `windows-rs` repository `cargo run /home/test/windows-rs`
- Go to WinProject(which is inside this project) `cd WinProject/`
- Compile this project `wine cargo build` (first time, compilation can take several hours, so I suggest to use prebuilt binaries)
- Generated binaries should be available here `target/debug/wine_project.exe`
- By default, binary should not crash, because all problematic functions are excluded in `settings.rs` file, so feel free to comment required lines.
- Setup if needed `settings.txt` file, template is already available in path `WinProject/settings.txt`

## Reproducer
Reproducer code is available in `Reproducer` folder.  
It is used to automatically test all functions, without needing to manually check them.   
To run it, needed is to disable(only in Wine) crash dialog `winetricks nocrashdialog`.  
App require adding 3 arguments:
- Path to fuzzer - compiled windows binary from `WinProject` folder
- System - Linux/Windows - it is needed, to be able to run wine/exe file directly `wine app.exe`/`app.exe`
- Type of Check - Fuzzer/Reproducible, fuzzer tests functions with random arguments, reproducible tests with same arguments every time

## How to help?
Project already is mostly done, so I think that the biggest effort should be put to improving Wine.

### Implementing unimplemented functions
The most basic way to help is to implement some missing functions in Wine.  
The easiest way is to change `stub` to `stdcall` with parameters and create empty functions with FIXME message(in this patch it is only 9 lines of code) - https://bugs.winehq.org/attachment.cgi?id=72032&action=diff

Info about required parameters can be taken directly from Windows docs site - https://docs.microsoft.com/en-us/windows/win32/api/setupapi/nf-setupapi-setupqueryinfversioninformationw
### Implementing unimplemented dll files
Currently, there are thousands of not checked functions by this fuzzer, because Wine not support some dlls.  
Implementing them require to list all functions from inside(probably some tools to extract this exists), but except that, it looks not so hard to create - https://source.winehq.org/git/wine.git/commitdiff/b45ac3a0b8a007de631534085ef489f8574b98f9    
List of missing dlls, that this project tried to use:
```
dsparse.dll, dsprop.dll, msajapi.dll, mspatchc.dll, winbio.dll, certadm.dll, cldapi.dll, ntlanman.dll, icm32.dll, 
api-ms-win-core-comm-l1-1-1.dll, api-ms-win-core-comm-l1-1-2.dll, infocardapi.dll, wdstptc.dll, wsclient.dll, dhcpsapi.dll, 
diagnosticdataquery.dll, windows.ui.xaml, srpapi.dll, efswrt.dll, wecapi.dll, eappprxy.dll, eappcfg.dll, fxsutility.dll, 
winfax.dll, fhsvcctl.dll, clfsw32.dll, txfw32.dll, wofutil.dll, api-ms-win-gaming-expandedresources-l1-1-0.dll, bcp47mrm.dll, 
icu.dll, gpedit.dll, computenetwork.dll, computecore.dll, computestorage.dll, vmsavedstatedumpprovider.dll, 
tokenbinding.dll, iscsidsc.dll, chakra.dll, magnification.dll, api-ms-win-core-memory-l1-1-8.dll, mdmlocalmanagement.dll, 
mdmregistration.dll, imgutil.dll, msrating.dll, netsh.dll, cscapi.dll, p2p.dll, projectedfslib.dll, rpcns4.dll, rtm.dll, wscapi.dll, 
sensorsutilsv2.dll, winsqlite3.dll, api-ms-win-wsl-api-l1-1-0.dll, api-ms-win-core-sysinfo-l1-2-4.dll, 
api-ms-win-core-sysinfo-l1-2-3.dll, ualapi.dll, ntlanman.dll, windows.networking, mrmsupport.dll, 
ondemandconnroutehelper.dll, api-ms-win-net-isolation-l1-1-0.dll, wnvapi.dll, wldp.dll, api-ms-win-core-realtime-l1-1-2.dll, 
api-ms-win-core-apiquery-l2-1-0.dll, webauthn.dll, xinputuap.dll, keycredmgr.dll, dssec.dll, rpcproxy.dll, 
licenseprotection.dll, inkobjcore.dll, verifier.dll, dxcompiler.dll, vertdll.dll, ndfapi.dll, wsmsvc.dll, vertdll.dll, rometadata.dll, 
mi.dll, vertdll.dll
```

### Fixing crashes
This is a lot of functions that crashes on Linux but not on Windows.

Fixing this invalid behavior would fix some apps, but sadly bug reports about it are closed by Wine devs because there is not yet found app that use such function and have problems - https://bugs.winehq.org/show_bug.cgi?id=52694

I understand that, but still I think that it is better to fix such thing before finding app that it use, because not everybody reports wine bugs.

## TODO
I think about possibility of collecting results of executed functions, so it would allow to compare it with Windows, and found even more bugs.