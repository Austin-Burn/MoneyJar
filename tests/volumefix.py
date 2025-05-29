from pycaw.pycaw import AudioUtilities, IAudioEndpointVolume
from ctypes import cast, POINTER
from comtypes import CLSCTX_ALL
import time
import keyboard

devices = AudioUtilities.GetSpeakers()
interface = devices.Activate(IAudioEndpointVolume._iid_, CLSCTX_ALL, None)
volume = cast(interface, POINTER(IAudioEndpointVolume))

run = True
volume_level = 0.0
# Set volume: 0.0 = min, 1.0 = max
while run:
    volume.SetMasterVolumeLevelScalar(volume_level, None) 
    time.sleep(0.1)
    if keyboard.is_pressed("up"):
        volume_level += 0.1
        print(volume_level)
    elif keyboard.is_pressed("down"):
        volume_level -= 0.1
        print(volume_level)
    elif keyboard.is_pressed("esc"):
        run = False

