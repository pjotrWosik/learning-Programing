static void Main()
{
    Random losowanie = new Random();

    int wylosowana = losowanie.Next(1, 101);
    int ile_prub = 6;
    int pruba = 0;

    
    
    Console.Write("masz 6 prub zgadnij liczbe pomiedzy 1 a 100");
    while (wylosowana != pruba)
    {
        Console.WriteLine("Podaj swuj numer");
        int.TryParse(Console.ReadLine(), out pruba);

        if (pruba == wylosowana)
        {
            Console.WriteLine("wygrałes brawo ma nyga");
            break;
        }
        else if (ile_prub == 0) 
        {
            Console.WriteLine("przegrałes"); 
            break;
        }
        else if (pruba > wylosowana)
        {
            Console.WriteLine("zadurzo");
            ile_prub --;
        }
        else if (pruba < wylosowana)
        {
            Console.WriteLine("zamalo");
            ile_prub --;
        }
    }
}
