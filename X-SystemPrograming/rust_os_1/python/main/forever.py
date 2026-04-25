import os
import sys
import subprocess
import signal

if len(sys.argv) < 2:
    print("use:\npython3 forever.py <cmd> [args]")
    sys.exit(1)

cmd = sys.argv[1:]
file_out = open("output.log", "a")
file_err = open("error.log", "a")

signal_num = 0
is_interrupt = False

try:
    bg_process = subprocess.Popen( cmd, stdin=subprocess.DEVNULL, stdout=file_out, stderr=file_err)
    # Write registry
    with open("registry.txt", "w") as f:
        f.write(f"{bg_process.pid} {' '.join(cmd)}\n")

    print(f"Forever PID {os.getpid()} started child PID {bg_process.pid}")

except Exception as e:
    print(f"Failed to start background process: {e}")
    file_out.close()
    file_err.close()
    sys.exit(1)


def kill_process(signum, frame):
    global signal_num, is_interrupt
    is_interrupt = True
    signal_num = signum
    print(f"\nReceived signal {signal_num}. Sending SIGTERM to {bg_process.pid}")
    bg_process.terminate()


signal.signal(signal.SIGTERM, kill_process)
signal.signal(signal.SIGINT, kill_process)

try:
    bg_process.wait()
except InterruptedError:
    pass

file_out.close()
file_err.close()

if is_interrupt:
    print(f"Process stopped via signal {signal_num}")
else:
    print("Process exited naturally")

print(f"Exiting forever.py {os.getpid()}")
