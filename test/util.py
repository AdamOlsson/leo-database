import getopt, sys, traceback


def parseReqHead(head=str) -> dict:
    headers = []
    for h in head.splitlines():
        if ':' in h:
            headers.append(map(str.strip, h.split(':', 1)))

    headers = dict(headers)
    headers["boundary"] = headers["Content-Type"].split("boundary=")[1]

    return headers

def parseBodyHead(head=str) -> dict:
    headers = []
    for h in head.splitlines():
        if ':' in h:
            headers.append(map(str.strip, h.split(':', 1)))

    return dict(headers)

def getBody(sock, bodysize):
    data = b""
    chunk = b""
    bytes_read = 0
    while bytes_read < bodysize:
        chunk = sock.recv(4096)
        bytes_read += 4096
        data += chunk

    return data

def getHead(sock):
    data = b""
    while not data.endswith(b"\r\n\r\n"):
        data += sock.recv(1)
    return data


def readArgs():
    def printHelp():
        print("Usage:\npython test/client.py [OPTIONS]")
        print("\n")
        print("[OPTIONS]")
        FORMAT = "{:<10} {:<10} {:<10} {:<10}"
        # LONG SHORT REQURED DESCRIPTION
        print(FORMAT.format("LONG", "SHORT", "REQUIRED", "DESCRIPTION"))
        print(FORMAT.format("--port", "-p", "yes", "Specifies the port number the client will try connecting to."))
        print("\n\n")
    try:
        opts, args = getopt.getopt(sys.argv[1:],"h:p:",["port"])
        if len(opts) == 0:
            raise ValueError("Required arguments not specified.")
    except:
        traceback.print_exc()
        sys.exit(2)
    
    flags = {}
    for opt, arg in opts:
        if opt in ("-h", "--help"):
            printHelp()
            sys.exit(0)
        elif opt in ("-p", "--port"):
            flags["port"] = int(arg)

    if not flags["port"]:
        raise ValueError("Port is not specified.")

    return flags