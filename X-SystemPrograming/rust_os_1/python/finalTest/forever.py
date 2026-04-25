import os
import sys
import subprocess
import signal


if len(sys.argv) < 2:
    print("format")
    sys.exit(1)

cmd = sys.argv[1:]
file_out = open("out.log","a")
file_err = open("err.log","a")
signal_no = 0
is_intrupted = False

try:
    bg_process = subprocess.Popen(cmd, stdin=subprocess.DEVNULL,stdout=file_out,stderr=file_err)
    with open("reg.txt","w") as w:
        w.write(f"{bg_process.pid} {' '.join(cmd)}\n")
    print("started")
    
except:
    print("- BG_process failed")
    file_out.close()
    file_err.close()
    sys.exit(1)
def manual_stop(signum, current_stack):
    global signal_no, is_intrupted
    is_intrupted = True
    signal_no = signum
    print("\n- Stoping Process")
    bg_process.terminate()

signal.signal(signal.SIGINT, manual_stop)
signal.signal(signal.SIGTERM, manual_stop)

bg_process.wait()
file_out.close()
file_err.close()

if is_intrupted:
    print(f"- Process termintated with signal no {signal_no}")
else:
    print(f"- Process died naturally")

print("- Forever.py endign.")



