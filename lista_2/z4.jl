#=
Maksmilian Neumann
=#

#importujemey biblioteke do modelowania i solver
using JuMP
using HiGHS

#zasięg widzenia kamer
k = 2
#rozsatwienie kontenerów na placu
c = [
    0 1 0 0 0
    0 0 0 1 0
    1 0 0 0 1
    0 1 0 1 0
    0 1 1 0 0
]
#wysokość planszy
N = size(c)[1]
#szerokość planszy
M = size(c)[2]
#tworzymy model
model = Model(HiGHS.Optimizer)
#towrzymy zmienno jakonmatrix 2d w rozmiarze naszego placu na któej albo jest kamera 1 albo nie ma 0
@variable(model, x[1:N, 1:M], Bin)
#tworzymy nasz cel (minimalizacaj ilości kamer)
@objective(model, Min, sum(x))
#ograniczenie zadna kamera nie moze byc na tym samym polu co kontener
@constraint(model, [n in 1:N, m in 1:M], x[n, m] + c[n, m] <= 1)
for n in 1:N, m in 1:M
    #sprawdzamy czy robić ograniczenie(czy jest na tym miejcu kontener)
    if c[n, m] == 1 
        #odległośćw pionie gdzie powinna znajdować sie conajmniej jedna kamera od kontenera
        row_range = max(1, n - k):min(N, n + k)
        #odległośćw poziomie gdzie powinna znajdować sie conajmniej jedna kamera od kontenera
        col_range = max(1, m - k):min(M, m + k)
        #okraniczenie ze k miesc w góre dół prawo i lewo musi znajdować sie conajmniej jendna kamera dla kontenera
        @constraint(model, (sum(x[row_range, m]) + sum(x[n, col_range])) >= 1) 

    end
end
#optymalizujemy model
optimize!(model)
#wyświetlamy rozmeszczenie kamer
display(round.(Int, value.(x)))
#wyświetlamy rozmeszczenie kontenerów
display(value.(c))

