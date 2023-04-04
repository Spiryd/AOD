using JuMP
import HiGHS
import LinearAlgebra

N = 8 #wysokość
M = 8 #szerokość
k = 8 #odległość widzenia kamer

function pole_widzenia(kamery, n, m)
    return 1
end

model = Model(HiGHS.Optimizer)
@variable(model, kamery[1:N, 1:M], Bin);
@variable(model, kontenery[1:N, 1:M], Bin);
for m in 1:M
    for n in 1:N
        @constraint(model, (kamery[n, m] + kontenery[n, m]) <= 1)
    end
end
for m in 1:M
    for n in 1:N
        @constraint(model, (kontenery[n, m] + pole_widzenia(kamery, n, m)) == 2)
    end
end
@objective(model, Max, sum(kontenery));
optimize!(model)
solution_summary(model)
solution = round.(Int, value.(kontenery))
