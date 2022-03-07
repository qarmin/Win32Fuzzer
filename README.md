# Win32 Fuzzer

This repository contains fuzzer to test Windows API functions(Win32 API).

I created it to help Wine to better mimic Windows OS and allow running more Windows software on Linux. 

Looks that currently tests - Classes: "165", Functions: "10540", Arguments: "35174"

This project works fine with `windows-rs 0.33.0`, so it may require rewrite to support newer versions since