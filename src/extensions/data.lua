
-- aaaa-mm-dd
return {
    prefixo = "data_",

    
    insert = function(chave, valor)
        local ano, mes, dia = string.match(valor, "^(%d%d%d%d)-(%d%d)-(%d%d)$")
        if ano == nil or mes == nil or dia == nil then
            return false, "Formato de data inválido, use aaaa-mm-dd"
        end

        local ano_num = tonumber(ano)
        local mes_num = tonumber(mes)
        local dia_num = tonumber(dia)
        if mes_num < 1 or mes_num > 12 then
            return false, "Mês inválido, use um mês entre 01 e 12"
        end

        if dia_num < 1 or dia_num > 31 then
            return false, "Dia inválido, use um dia entre 01 e 31"
        end

        if mes_num == 2 then
            local bissexto = (ano_num % 4 == 0 and ano_num % 100 ~= 0) or (ano_num % 400 == 0)
            if bissexto and dia_num > 29 then
                return false, "Dia inválido para fevereiro em ano bissexto"
            elseif not bissexto and dia_num > 28 then
                return false, "Dia inválido para fevereiro em ano não bissexto"
            end
        elseif mes_num == 4 or mes_num == 6 or mes_num == 9 or mes_num == 11 then
            if dia_num > 30 then
                return false, "Dia inválido para o mês informado"
            end
        end
        
        return true, valor
    end,

    select = function(chave, valor)
        local ano, mes, dia = string.match(valor, "^(%d%d%d%d)-(%d%d)-(%d%d)$")
        local formatado = dia .. "/" .. mes .. "/" .. ano
        return true, formatado
    end
}