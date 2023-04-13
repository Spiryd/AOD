#=
Maksmilian Neumann
=#

#importujemey biblioteki do modelowania i obsługi JSON oraz solver 
using JuMP
using HiGHS
import JSON

#dane
data = JSON.parse("""
{
    "machines": {
        "M1": {"costs": 2},
        "M2": {"costs": 2},
        "M3": {"costs": 3}
    },
    "products": {
        "p1": {"demand": 400, "price": 9, "costs": 4},
        "p2": {"demand": 100, "price": 7, "costs": 1},
        "p3": {"demand": 150, "price": 6, "costs": 1},
        "p4": {"demand": 500, "price": 5, "costs": 1}
    },
    "processing": {
        "M1 => p1": 5,
        "M1 => p2": 3,
        "M1 => p3": 4,
        "M1 => p4": 4,
        "M2 => p1": 10,
        "M2 => p2": 6,
        "M2 => p3": 5,
        "M2 => p4": 2,
        "M3 => p1": 6,
        "M3 => p2": 4,
        "M3 => p3": 3,
        "M3 => p4": 1
    }
}
""")
#klucze do maszyn i produktów
M = keys(data["machines"])
P = keys(data["products"])
#funkcja do odnajdywania czasu obróbki w min/kg
processing_times(m::String, p::String) = data["processing"]["$(m) => $(p)"]
#tworzymy model
model = Model(HiGHS.Optimizer)
#tworzymy zmienna czasy urzytkowania maszyn dla poszczególnych produktów w minutach
@variable(model, x[M, P] >= 0, Int)
#zadna maszyna nie moze pracować ponad 60 h
@constraint(model, [m in M], sum(x[m, :]) <= (60*60))
#dla kazdego produktu nie mozna wyprodukować więcej niz popyt
for p in P 
    @constraint(model, sum(x[m, p] * (processing_times(m, p)^-1) for m in M) <= data["products"][p]["demand"])   
end
#maksymalizujemy profit
@objective(model, Max, sum((x[m, p] * (processing_times(m, p)^-1) * (data["products"][p]["price"] - data["products"][p]["costs"]) - ((x[m, p]/60) * data["machines"][m]["costs"])) for m in M, p in P))
#optymalizujemy model
optimize!(model)
#printujemy wyniki
println("profit = ", objective_value(model))
println("kg of products made:")
for p in P 
    println(p,  " => ", value.(sum(x[m, p] * (processing_times(m, p)^-1) for m in M))) 
end
println("schedule:")
for m in M, p in P
    if  !(value(x[m, p]) ≈ 0 )    
        println(m, " => ", p, ": ", value(x[m, p])) 
    end
end
