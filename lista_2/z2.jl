using JuMP
using HiGHS

# liczba miast
n = 5

# macierz kosztów przejazdu
c = [0 5 10 0 0;
     0 0 4 11 0;
     0 0 0 2 7;
     0 0 0 0 3;
     0 0 0 0 0]

# macierz czasów przejazdu
t = [0 1 3 0 0;
     0 0 2 4 0;
     0 0 0 1 2;
     0 0 0 0 3;
     0 0 0 0 0]

# miasta początkowe i końcowe
i = 1
j = 5

# ograniczenie czasowe
T = 10

# tworzenie modelu
m = Model(HiGHS.Optimizer)

# dodawanie zmiennych decyzyjnych
@variable(m, x[1:n, 1:n], Bin)

# dodawanie ograniczenia czasowego
@constraint(m, sum(x.*t) <= T)

# dodawanie ograniczenia budżetowego
@constraint(m, sum(x) <= n-1)

# dodawanie ograniczeń dla miast początkowego i końcowego
for k = 1:n
    if k != i
        @constraint(m, sum(x[i,k]) - sum(x[k,i]) == 0)
    end
    if k != j
        @constraint(m, sum(x[k,j]) - sum(x[j,k]) == 0)
    end
end

# funkcja celu
@objective(m, Min, sum(x.*c))

optimize!(m)

value.(x)
