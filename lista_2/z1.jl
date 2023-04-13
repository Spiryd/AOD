#=
Maksmilian Neumann
=#

#importujemey biblioteki do modelowania i obsługi JSON oraz solver 
using JuMP
using HiGHS
import JSON

#dane do zadania
data = JSON.parse("""
{
    "firmy": {
        "Firma1": {"capacity": 275},
        "Firma2": {"capacity": 550},
        "Firma3": {"capacity": 660}
    },
    "lotniska": {
        "Lotnisko1": {"demand": 110},
        "Lotnisko2": {"demand": 220},
        "Lotnisko3": {"demand": 330},
        "Lotnisko4": {"demand": 440}
    },
    "ceny": {
        "Firma1 => Lotnisko1": 10,
        "Firma1 => Lotnisko2": 10,
        "Firma1 => Lotnisko3": 9,
        "Firma1 => Lotnisko4": 11,
        "Firma2 => Lotnisko1": 7,
        "Firma2 => Lotnisko2": 11,
        "Firma2 => Lotnisko3": 12,
        "Firma2 => Lotnisko4": 13,
        "Firma3 => Lotnisko1": 8,
        "Firma3 => Lotnisko2": 14,
        "Firma3 => Lotnisko3": 4,
        "Firma3 => Lotnisko4": 9
    }
}
""")
#firmy
F = keys(data["firmy"])
#lotniska
L = keys(data["lotniska"])
#funkcja ceny od firmy do lotniska
price(f::String, l::String) = data["ceny"]["$(f) => $(l)"]
#tworzymy model
model = Model(HiGHS.Optimizer)
#definiujemy zmienną ilość paliwa dostarczanego z firmy f do lotniska l w postaci 2d matrycy
@variable(model, x[F, L] >= 0)
#ograniczenie ze kazda firma nie moze dostarzyc wiecej paliwa niz jest w stanie
@constraint(model, [f in F], sum(x[f, :]) <= data["firmy"][f]["capacity"])
#ograniczenie ze kazda lotnisko musi mniec conajmiejmniej spelnione swoje zapotrzebowanie
@constraint(model, [l in L], sum(x[:, l]) >= data["lotniska"][l]["demand"])
#ustawiamy cel jako minimalizacja kosztów zakupu paliwa
@objective(model, Min, sum(price(f, l) * x[f, l] for f in F, l in L));
#optymalizujemy model
optimize!(model)
solution_summary(model)
#wyświetlamy wyniki
for f in F, l in L
    if  !(value(x[f, l]) ≈ 0 )    
        println(f, " => ", l, ": ", value(x[f, l])) 
    end
end

value(objective_value(model))
