import random

losowana = random.randint(1, 100)
liczba_prub = 6
pruba = 0

while pruba != losowana and liczba_prub > 0:
    pruba = int(input("wybierz liczbe miedzy 1 a 100"))
    if pruba == losowana:
        print("WYgrałeś brawoo")
    elif pruba > losowana:
        print("liczba jest za durza sprubuj ponownie")
        liczba_prub -= 1
        print(f"ile prub ci zostastalo: {liczba_prub}")
    elif pruba < losowana:
        print("liczba jest za mala sprubuj ponownie")
        liczba_prub -= 1
        print(f"ile prub ci zostastalo: {liczba_prub}")
if pruba != losowana:
    print(f"Przegrałes magiczna liczba to: {losowana}")
