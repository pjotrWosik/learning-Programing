import random

pyt1 = {
    "pytanie": "costam1",
    "odpowiedz": [("A. ", True), ("B. ", False), ("C.", False)]
}
pyt2 = {
    "pytanie": "costam2",
    "odpowiedz": [("A. ", True), ("B. ", False), ("C.", False)]
}
pytania = {
    "pytanie1": pyt1,
    "pytanie2": pyt2,
}

print("bedziesz grac w gre gdzie muszisz odpowiedziec na losowe pytanie")
print("bedziesz odpowiadac a, b, c\n startooo!!!! ")

los_pytas = random.choice(list(pytania.values()))

print(f"{los_pytas["pytanie"]}")
for text, _ in los_pytas["odpowiedz"]:
    print(text)

odp = input("wybierz jedno z odpowiedzi wpisujac a, b lub c: \n").strip().upper()

for text, poprawna in los_pytas["odpowiedz"]:
    if text.startswith(odp) and poprawna:
        print("dobra odpowiedz")
        break
    if text.startswith(odp) and not poprawna:
        print("zla odpowied")
        break
