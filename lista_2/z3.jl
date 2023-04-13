using JuMP, HiGHS

# Minimalne liczby radiowozów dla każdej zmiany i dzielnicy
min_vehicles = [2 4 3;
                3 6 5;
                5 7 6]

# Maksymalne liczby radiowozów dla każdej zmiany i dzielnicy
max_vehicles = [3 7 5;
                5 7 10;
                8 12 10]

# Minimalne liczby radiowozów dla każdej dzielnicy
min_districts = [10, 14, 13]

# Minimalne liczby radiowozów dla każdej zmiany
min_shifts = [10, 20, 18]

m = Model(HiGHS.Optimizer)

@variable(m, x[1:3,1:3,1:3] >= 0, Int)

# Suma x[i,j,k] po i (zmianie) musi być większa bądź równa min_shifts[k]
for k = 1:3
    @constraint(m, sum(x[i,j,k] for i=1:3, j=1:3) >= min_shifts[k])
end

# Suma x[i,j,k] po j (dzielnicy) musi być większa bądź równa min_districts[j]
for j = 1:3
    @constraint(m, sum(x[i,j,k] for i=1:3, k=1:3) >= min_districts[j])
end

# Suma x[i,j,k] po k (zmianie) musi być mniejsza bądź równa max_vehicles[j,k]
for i = 1:3, j = 1:3
    @constraint(m, sum(x[i,j,k] for k=1:3) <= max_vehicles[i,j])
end

# Suma x[i,j,k] po i (zmianie) musi być większa bądź równa min_vehicles[j,k]
for j = 1:3, k = 1:3
    @constraint(m, sum(x[i,j,k] for i=1:3) >= min_vehicles[j,k])
end

# Warunki cyrkulacji
for i = 1:3, j = 1:3
    @constraint(m, sum(x[i,j,k] for k=1:3) == sum(x[i,j,l] for l=1:3))
end

# Minimalizujemy całkowitą liczbę radiowozów
@objective(m, Min, sum(x))

optimize!(m)

# Wyświetlamy wynik
println("Minimalna liczba radiowozów: ", objective_value(m))
println("Przydział radiowozów:")
for k = 1:3
    println("Zmiana ", k, ":")
    for j = 1:3
        println("\tDzielnica ", j, ": ", sum(value(x[i,j,k]) for i=1:3))
    end
end
