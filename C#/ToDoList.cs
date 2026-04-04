class Todo
{
    public string Nazwa { get; set; }
    public bool Gotowe { get; set; }
}

internal class Todolist
{
    static string MenuGluwne()
    {
        Console.WriteLine("Witaj w Liscie ToDo");
        Console.WriteLine("Wybierz operacje jaka chcesz wykonac");
        Console.WriteLine("Dodaj zadanie = 1");
        Console.WriteLine("wyswietl Liste = 2");
        Console.WriteLine("Odznacz gotowe = 3");
        Console.WriteLine("Usun zadanie = 4");
        Console.WriteLine("Wyjdz z programu = 0");

        Console.WriteLine("twuj wybur to: ");
        return Console.ReadLine();
    }

    static void Dodaj(List<Todo> lista)
    {
        Console.Write("Podaj nazwa zadania: ");
        string nazwa = Console.ReadLine();
        lista.Add(new Todo {Nazwa = nazwa, Gotowe = false});
        Console.WriteLine("dodano");
    }

    static void Wyswietl(List<Todo> lista)
    {
        for (int i = 0; i < lista.Count; i++)
        {
            string status = lista[i].Gotowe ? "✓" : "✗";
            Console.WriteLine($"{i + 1}. [{status}] {lista[i].Nazwa}");
        }
    }

    static void Odznacz(List<Todo> lista)
    {
        Wyswietl(lista);
        Console.WriteLine("Ktury numer?: ");
        int nr = int.Parse(Console.ReadLine()) - 1;
        lista[nr].Gotowe = true;
    }

    static void Usun(List<Todo> lista)
    {
        Wyswietl(lista);
        Console.WriteLine("Ktury numer usunąc?: ");
        int nr = int.Parse(Console.ReadLine()) - 1;
        lista.RemoveAt(nr);
        Console.WriteLine("Usunieto!");
    }

    static void Main()
    {
        List<Todo> lista = new List<Todo>();

        while(true)
        {
            string wybur = MenuGluwne();

            if (wybur == "0") break;
            else if (wybur == "1") Dodaj(lista);
            else if (wybur == "2") Wyswietl(lista);
            else if (wybur == "3") Odznacz(lista);
            else if (wybur == "4") Usun(lista);
        }
    }
}
