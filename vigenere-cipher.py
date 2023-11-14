from itertools import cycle
from typing import Tuple

user_input = input("Encode Message(1), Decode Message(2): ")

if user_input == "1":
    encode = True
elif user_input == "2":
    encode = False
else:
    print("Invalid input. Please enter 1 or 2.")
    exit(1)

cipher = input("Cipher: ")
key = input("Key: ")


def vigenere(x: Tuple[str, str]) -> str:
    [c, k] = x
    shift = ord(k.lower()) - ord('a')
    operation = shift if encode else -shift
    if 'a' <= c <= 'z':
        c = chr(ord('a') + (ord(c) - ord('a') + operation) % 26)
    elif 'A' <= c <= 'Z':
        c = chr(ord('A') + (ord(c) - ord('A') + operation) % 26)
    return c


text = "".join(map(vigenere, zip(cipher, cycle(key))))
print(f"Text: {text}")
