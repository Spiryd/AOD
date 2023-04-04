using JuMP
import HiGHS
import LinearAlgebra

N = 8 #wysokość
M = 8 #szerokość
k = 8 #odległość widzenia kamer

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
        @constraint(model, (kontenery[n, m] + (sum(kamery[n, :]) > 0) || (sum(kamery[:, m]) > 0)) == 2)
    end
end
@objective(model, Max, sum(kontenery));
optimize!(model)
solution_summary(model)
solution = round.(Int, value.(kontenery))
