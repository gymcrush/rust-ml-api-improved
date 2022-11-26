
import base64
import json

try:
    import requests
except:
    print("// -----------------------\n[requirement] pip install requests\n// --------------------------")
    raise


def request_image(img):
    print('request to localhost:3000')