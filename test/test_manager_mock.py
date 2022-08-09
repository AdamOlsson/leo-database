import socket, os
import util as util
os.chdir(os.path.join(os.getcwd(), os.path.dirname(__file__))) # change to working dir to test/ 

HTTPOK = b'HTTP/2 200 OK\r\nContent-Type: text/html; encoding=utf8\r\nContent-Length: 0\r\nConnection: close\r\n'

ERROR_FORMAT = "\n\
#####################\n\
# ERROR: \n\
# Expected {} in response but found:\n\
# {}\n\
#####################"
def print_error(expected_code, rsp):
    print(ERROR_FORMAT.format(expected_code, rsp))

def test_video_post(address, port):
    print("Running test_video_port test...", end="")
    filename = "output.mp4"
    video = None
    with open("resource/{}".format(filename), "rb") as f:
        video = f.read()

    boundary = b'???'
    postbodyheaders = b'Content-Disposition: form-data; name="video"; filename="output_client.mp4"\r\nContent-Type: application/octet-stream'
    postbody = postbodyheaders + b'\r\n\r\n' + video + b'\r\n' + boundary + b'--\r\n'

    postbodylength = bytes(str(len(postbody)), 'utf-8')
    postreq = b'POST /video HTTP/2\r\nContent-Length: ' + postbodylength + b'\r\nContent-Type: multipart/form-data; boundary=' + boundary + b'\r\nExpect: 100-continue\r\n\r\n'

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((address, port))
        # Send initial POST request
        s.send(postreq)

        expect_continue_100 = s.recv(1024)
        if not (b'100' in expect_continue_100):
            print_error("100", expect_continue_100)
            exit(1)

        # Send POST req body
        s.send(postbody)
        expect_ok_200 = s.recv(1024)
        if not (b'200' in expect_ok_200):
            print_error("200", expect_continue_100)
            exit(1)

    print("OK!")

if __name__ == "__main__":
    flags = util.readArgs()
    port = flags["port"]

    test_video_post("0.0.0.0", port)