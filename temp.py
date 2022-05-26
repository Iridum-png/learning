string = "Hello World"
new = ""

for i, char in enumerate(string):
    if char.isupper():
        new += char.lower()
    else:
        new += char.upper()

print(new)
