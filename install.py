import requests
import os

url = 'https://kitten.tronlang.org/v/latest'
response = requests.get(url)
open('kitten.exe', 'wb').write(response.content)

os.system('move kitten.exe %USERPROFILE%\\AppData\\Local\\Microsoft\\WindowsApps\\')