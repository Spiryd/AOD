using JuMP
using HiGHS
import JSON

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

F = keys(data["firmy"])
L = keys(data["lotniska"])
price(f::String, l::String) = data["ceny"]["$(f) => $(l)"]

model = Model(HiGHS.Optimizer)
@variable(model, x[F, L] >= 0)
@constraint(model, [f in F], sum(x[f, :]) <= data["firmy"][f]["capacity"])
@constraint(model, [l in L], sum(x[:, l]) >= data["lotniska"][l]["demand"])
@objective(model, Min, sum(price(f, l) * x[f, l] for f in F, l in L));

optimize!(model)
solution_summary(model)

for f in F, l in L
    if  !(value(x[f, l]) ≈ 0 )    
        println(f, " => ", l, ": ", value(x[f, l])) 
    end
end
