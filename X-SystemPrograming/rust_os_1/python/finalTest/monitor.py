
import os 
import sys
import subprocess
import time

REG_FILE = "reg.txt"
SLEEP_INTERVAL = 3

def read_reg():
    if not os.path.exists(REG_FILE):
        return None, None
    with open(REG_FILE, "r") as r:
        line = r.readline().strip()
    if not line:
        return None, None

    parts = line.split()
    pid = int(parts[0])
    cmd = parts[1:]
    return pid, cmd

def if_process_alive(pid):
    try:
        os.kill(pid, 0)
        return True
    except PermissionError:
        return True
    except ProcessLookupError:
        return False

def restart_process(cmd):
    print(f"restarting process {' '.join(cmd)}")
    process = subprocess.Popen(["python3","forever.py"]+cmd )

    with open(REG_FILE,"w") as w:
        w.write(f"{process.pid} {' '.join(cmd)}\n")
    return process.pid


def main():

    while True:
        pid, cmd = read_reg()
        if pid is None:
            time.sleep(SLEEP_INTERVAL)
            continue
        if not if_process_alive(pid):
            print("dead")
            restart_process(cmd)
        time.sleep(SLEEP_INTERVAL)


#---------------------------------------
if __name__ == "__main__":
    main()
