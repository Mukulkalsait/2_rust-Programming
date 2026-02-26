import os
import sys
import time
import subprocess
import signal

REG_FILE = "registry.txt"
INTERVAL = 3


# ------------------------
# FOREVER MODE
# ------------------------

def run_forever(cmd):
    file_out = open("output.log", "a")
    file_err = open("error.log", "a")

    bg_process = subprocess.Popen(
        cmd,
        stdin=subprocess.DEVNULL,
        stdout=file_out,
        stderr=file_err
    )

    # Write registry
    with open(REG_FILE, "w") as f:
        f.write(f"{bg_process.pid} {' '.join(cmd)}\n")

    print(f"Started child {bg_process.pid}")

    def handle_signal(signum, frame):
        print(f"Received signal {signum}, terminating child...")
        bg_process.terminate()

    signal.signal(signal.SIGTERM, handle_signal)
    signal.signal(signal.SIGINT, handle_signal)

    bg_process.wait()

    file_out.close()
    file_err.close()
    print("Forever exiting.")


# ------------------------
# MONITOR MODE
# ------------------------

def read_registry():
    if not os.path.exists(REG_FILE):
        return None, None

    with open(REG_FILE, "r") as f:
        line = f.readline().strip()

    if not line:
        return None, None

    parts = line.split()
    pid = int(parts[0])
    cmd = parts[1:]
    return pid, cmd


def process_alive(pid):
    try:
        os.kill(pid, 0)
        return True
    except ProcessLookupError:
        return False
    except PermissionError:
        return True


def run_monitor():
    print("Monitoring started...")

    while True:
        pid, cmd = read_registry()

        if pid is None:
            time.sleep(INTERVAL)
            continue

        if not process_alive(pid):
            print(f"Process {pid} died. Restarting...")
            subprocess.Popen(["python3", sys.argv[0], "start"] + cmd)

        time.sleep(INTERVAL)


# ------------------------
# ENTRY POINT
# ------------------------

if __name__ == "__main__":

    if len(sys.argv) < 2:
        print("Usage:")
        print("  python3 program.py start <cmd>")
        print("  python3 program.py monitor")
        sys.exit(1)

    mode = sys.argv[1]

    if mode == "start":
        if len(sys.argv) < 3:
            print("No command provided.")
            sys.exit(1)
        run_forever(sys.argv[2:])

    elif mode == "monitor":
        try:
            run_monitor()
        except KeyboardInterrupt:
            print("\nMonitor stopped.")

    else:
        print("Unknown mode.")
