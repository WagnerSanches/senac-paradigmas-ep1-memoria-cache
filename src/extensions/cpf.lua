return {
    prefixo = "cpf_",

    insert = function(chave, valor)

        local cpfs_invalidos = {
            "00000000000", "11111111111", "22222222222", "33333333333",
            "44444444444", "55555555555", "66666666666", "77777777777",
            "88888888888", "99999999999"
        }

        if string.match(valor, "%D") then
            return false, "O CPF deve conter apenas números!"
        end

        for _, cpf in ipairs(cpfs_invalidos) do
            if valor == cpf then
                return false, "O CPF é inválido!"
            end
        end

        if string.len(valor) ~= 11 then
            return false, "O CPF deve ter 11 digitos!"
        end

        local soma = 0
        local regra = 10
        for i = 1, 9 do
            local digito = tonumber(string.sub(valor, i, i))
            soma = soma + (digito * regra)
            regra = regra - 1
        end
        
        local rs = soma % 11

        local verificador
        if rs < 2 then
            verificador = 0
        else
            verificador = 11 - rs
        end

        local digito_real = tonumber(string.sub(valor, 10, 10))

        if verificador ~= digito_real then
            return false, "O CPF é inválido!"
        end

        -- segundo digito
        regra = 11
        soma = 0
        for i = 1, 10 do
            local digito = tonumber(string.sub(valor, i, i))
            soma = soma + (digito * regra)
            regra = regra - 1
        end

        rs = soma % 11
        
        if rs < 2 then
            verificador = 0
        else
            verificador = 11 - rs
        end

        local digito_real2 = tonumber(string.sub(valor, 11, 11))

        if verificador ~= digito_real2 then
            return false, "O CPF é inválido!"
        end

        local valor_extistente = database_find_by_value(valor)
        
        if valor_extistente ~= nil and valor_extistente ~= chave then
            return false, "CPF já cadastrado na chave '" .. valor_extistente .. "'"
        end

        return true, valor
    end,

    select = function(chave, valor)

        local parte1 = string.sub(valor, 1, 3)
        local parte2 = string.sub(valor, 4, 6)
        local parte3 = string.sub(valor, 7, 9)
        local parte4 = string.sub(valor, 10, 11)

        local formatado = parte1 .. "." .. parte2 .. "." .. parte3 .. "-" .. parte4
        return true, formatado
    end
}