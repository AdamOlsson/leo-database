from email import header
import socket, os
from util import getBody, parseBodyHead, parseReqHead
import util as util
os.chdir(os.path.join(os.getcwd(), os.path.dirname(__file__))) # change to working dir to test/ 

HTTP_CONTINUE = b'HTTP/2 100 CONTINUE\r\nContent-Type: text/html; encoding=utf8\r\nContent-Length: 0\r\nConnection: keep-alive\r\n'
HTTP_OK = b'HTTP/2 200 OK\r\nContent-Type: text/html; encoding=utf8\r\nContent-Length: 0\r\nConnection: close\r\n'

ERROR_FORMAT = "\n\
#####################\n\
# ERROR: \n\
# Expected {} in response but found:\n\
# {}\n\
#####################"
def print_error(expected_code, rsp):
    print(ERROR_FORMAT.format(expected_code, rsp))

def test_post_video(address, port):
    print("Running test_video_port test...", end="")
    filename = "test_post_video.mp4"
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


def test_get_video(address, port):
    print("Running test_video_port test...", end="")

    getreq = b'GET /video/output_client.mp4 HTTP/2\r\nContent-Length: 0\r\n\r\n'

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((address, port))

        s.send(getreq) # Request to get video

        expect_200_ok = s.recv(1024) # Receive OK, server will expect continue
        expect_200_ok_utf8 = expect_200_ok.decode('utf-8')
        headers = parseReqHead(expect_200_ok_utf8)
        if not (b'200' in expect_200_ok):
            print_error("200", expect_200_ok)
            exit(1)
        
        s.send(HTTP_CONTINUE)

        payload = getBody(s, int(headers["Content-Length"]))

        assert(len(payload) == int(headers["Content-Length"]))

        payload_head, payload_body = payload.split(b'\r\n\r\n')
        parseBodyHead(payload_head.decode('utf-8')) # Verify that payload headers can be parsed
        payload_body, _ = payload_body.split(b'\r\n' + bytes(headers["boundary"], 'utf-8') + b'--\r\n')

        with open("resource/test_get_video.mp4", "wb") as f:
            f.write(payload_body)
        
        s.send(HTTP_OK)

    print("OK!")


if __name__ == "__main__":
    flags = util.readArgs()
    port = flags["port"]

    test_post_video("0.0.0.0", port)
    test_get_video("0.0.0.0", port)
