using System;
using System.Diagnostics;

[DebuggerDisplay($"{{{nameof(GetDebuggerDisplay)}(),nq}}")]
internal class Kalkulator
{
    static (int, int) PoburDwuchLiczb()
    {
        Console.WriteLine("podaj pierwszą liczbe: ");
        int a = float.TryParse(Console.ReadLine());
       
        Console.WriteLine("podaj drugą liczbe: ");
        int b = float.TryParse(Console.ReadLine());
        return (a, b);
    }

    static void dodawanie()
    {
        var (a, b) = PoburDwuchLiczb();
        Console.WriteLine($"{a} + {b} = {a + b}");
    }

    static void odejmowanie()
    {
        var (a, b) = PoburDwuchLiczb();
        Console.WriteLine($"{a} - {b} = {a - b}");
    }

    static void mnorzenie()
    {
        var (a, b) = PoburDwuchLiczb();
        Console.WriteLine($"{a} * {b} = {a * b}");
    }

    static void dzielenie()
    {
        var (a, b) = PoburDwuchLiczb();
        Console.WriteLine($"{a} / {b} = {a / b}");
    }



    static void Main()
    {
        while (true)
        {
            Console.WriteLine("witaj w kalkulatorze obecne funkcjie to:");
            Console.WriteLine("1 dodawanie");
            Console.WriteLine("2 odejmowanie");
            Console.WriteLine("3 mnorzenie");
            Console.WriteLine("4 dzielenie");

            Console.WriteLine("\nWybierz numer: ");
            string wybur = Console.ReadLine();

            if (wybur == "0") break;
            else if (wybur == "1")dodawanie();
            else if (wybur == "2")odejmowanie();
            else if (wybur == "3")mnorzenie();
            else if (wybur == "4")dzielenie();

            else
            {
                Console.WriteLine("otrzymalen nieodpowiednią liczbe");
                Console.ReadLine();
                return;
            }

            Console.WriteLine("\nNaciśnij Enter aby zakończyć...");
            Console.ReadLine();
        }
        Console.Write("bye bye");
    }

    private string GetDebuggerDisplay()
    {
        return ToString();
    }
}
