-- Calcul de la fonction factorielle

-- Récursive
fact x = if x == 0 then 1 else fact (x - 1) * x

-- Pattern matching
fact 0 = 1
fact x = x * fact (x - 1)

-- Gardes
fact x
   | x > 1 = x * fact (x - 1)
   | otherwise = 1

-- Liste et intervalle
fac x = product [1..x]

-- Fonctions d'ordre supérieure
mapList f [] = []
mapList f (x:xs) = f x : mapList f xs

-- Listes en compréhension et évaluation paresseuse
take 10 [ (i,j) | i <- [1..], j <- [1..], i < j ]

