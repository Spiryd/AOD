using JuMP
using HiGHS

N = ["Wrocław", "Warszawa", "Kraków", "Gdańsk", "Poznań", "Szczeciń", "Białstok", "Bydgoszcz", "Olsztn", "Berlin"]
n = length(N)
A = [("Wrocław", "Warszawa"), ("Warszawa", "Gdańsk"), ("Warszawa", "Kraków"), ("Gdańsk", "Białstok"), ("Gdańsk", "Szczeciń"), ("Wrocław", "Poznań"), ("Gdańsk", "Bydgoszcz"), ("Bydgosz", "Poznań"), ("Olsztyn", "Białystok"), ("Berlin", "Wrocław")]
m = length(A)
c = []
t = []
