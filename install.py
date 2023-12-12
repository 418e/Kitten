import requests
import os

url = 'https://github.com/418e/Kitten/releases/download/0.1.0/kitten'
response = requests.get(url)
open('kitten.exe', 'wb').write(response.content)

os.system('move kitten.exe %USERPROFILE%\\AppData\\Local\\Microsoft\\WindowsApps\\')