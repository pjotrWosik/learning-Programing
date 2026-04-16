print("witaj w kalkulkulatorze BMI")
print("Bedziesz musił podac swoją mase i wysokosc")

m = float(input("Podaj jk Gruby jestes w kg \n"))
h = float(input("podj jak wysoki jestes w centymetrach \n"))/100

bmi = m / h ** 2

print(bmi)
if bmi < 18.5:
    print("niedowga idz cos jesc chlopie bo umrzesz")
elif bmi <= 24.9:
    print("Waga prawidlowa BRAWO!")
elif bmi <= 29.9:
    print("jestes zbyt za gruby idz shudnij")
else: 
    print("ty pskudny grubasie idz pływaj bo biegac nigdy niebedziesz")
