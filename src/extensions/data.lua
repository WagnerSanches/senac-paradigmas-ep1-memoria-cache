
-- aaaa-mm-dd
return {
    prefixo = "data_",

    
    insert = function(chave, valor)
        local ano, mes, dia = string.match(valor, "^(%d%d%d%d)-(%d%d)-(%d%d)$")
        
        return "recebi: " .. valor
    end
}