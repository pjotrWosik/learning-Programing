import math

class Calculator:
    def __init__(self):
        self.pamiec_historia=[]

    def dodawanie(self, liczba1, liczba2):
        liczba3 = liczba1 + liczba2
        print(f"wynik dodawania: {liczba1} + {liczba2} = {liczba3}")
        self.pamiec_historia.append(f"wynik dodawania: {liczba1} + {liczba2} = {liczba3}")

    def odejmowanie(self, liczba1, liczba2):
        liczba3 = liczba1 - liczba2
        print(f"wynik odejmowania: {liczba1} - {liczba2} = {liczba3}")
        self.pamiec_historia.append(f"wynik odejmowania: {liczba1} - {liczba2} = {liczba3}")

    def mnozenie(self, liczba1, liczba2):
        liczba3 = liczba1 * liczba2
        print(f"wynik mnożenia: {liczba1} * {liczba2} = {liczba3}")
        self.pamiec_historia.append(f"wynik mnożenia: {liczba1} * {liczba2} = {liczba3}")

    def dzielenie(self, liczba1, liczba2):
        if liczba1 or liczba2 == 0:
            print("niemorzna dzielic przez zero")
        else:
            liczba3 = liczba1 / liczba2
            print(f"wynik dzielenia: {liczba1} / {liczba2} = {liczba3}")
            self.pamiec_historia.append(f"wynik dzielenia: {liczba1} / {liczba2} = {liczba3}")


    def potegowanie(self, liczba1, liczba2):
        liczba3 = pow(liczba1, liczba2)
        print(f"wynik potegowania: {liczba1}  {liczba2} tych dwuch liczb to: {liczba3}")
        self.pamiec_historia.append(f"wynik potegowania: {liczba1}  {liczba2} tych dwuch liczb to: {liczba3}")

    def pierwiastkowanie(self, liczba1, liczba2):
        liczba3 = liczba1 ** liczba2
        print(f"wynik pierwiastkowania to: {liczba2}^{liczba1} = {liczba3}")
        self.pamiec_historia.append(f"wynik pierwiastkowania to: {liczba2}^{liczba1} = {liczba3}")

    def historia(self):
        if not self.pamiec_historia:
            print("Historia jest pusta!")
        else:
            print(":--------------------------------:")
            for dzialanie in self.pamiec_historia:
                print(dzialanie)
            print(":--------------------------------:")

    def wyjscie(self):
        print("program zostaje zamkniety")

    def pobierz_2_liczby(self):
        liczba1 = float(input("podaj 1 liczbe"))
        liczba2 = float(input("podaj 2 liczbe"))
        return liczba1, liczba2


kalk = Calculator()

while True:
    print(":--------------------------------:")
    print("wpisz:")
    print("     1 - dodawanie")
    print("     2 - odejmowanie")
    print("     3 - mnorzenie")
    print("     4 - dzielenie")
    print("     5 - potegowanie")
    print("     6 - pierwiastkowanie")
    print("     7 - Otworz Historie")
    print("     8 - wyjsc z programu")
    print(":--------------------------------:")

    wybur = input("Wybierz dzialanie: ")

    if wybur == "1":
        liczba1, liczba2 = kalk.pobierz_2_liczby()
        kalk.dodawanie(liczba1, liczba2)
    elif wybur == "2":
        liczba1, liczba2 = kalk.pobierz_2_liczby()
        kalk.odejmowanie(liczba1, liczba2)
    elif wybur == "3":
        liczba1, liczba2 = kalk.pobierz_2_liczby()
        kalk.mnozenie(liczba1, liczba2)
    elif wybur == "4":
        liczba1, liczba2 = kalk.pobierz_2_liczby()
        kalk.dzielenie(liczba1, liczba2)
    elif wybur == "5":
        liczba1, liczba2 = kalk.pobierz_2_liczby()
        kalk.potegowanie(liczba1, liczba2)
    elif wybur == "6":
        liczba1, liczba2 = kalk.pobierz_2_liczby()
        kalk.pierwiastkowanie(liczba1, liczba2)
    elif wybur == "7":
        kalk.historia()
    elif wybur == "8":
        kalk.wyjscie()
        break
