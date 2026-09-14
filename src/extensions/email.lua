return {
    prefixo = "email_",

    insert = function(chave, valor) 
        local usuario, dominio = string.match(valor, "^([^@]+)@([^@]+)$")

        if usuario == nil or dominio == nil then
            return false, "Formato de e-mail inválido, use o formato usuario@dominio.extensao"
        end

        local dominio_nome, extensao = string.match(dominio, "^([^%.]+)%.([^%.]+)$")

        if dominio_nome == nil or extensao == nil then
            return false, "Formato de e-mail inválido, use o formato usuario@dominio.extensao"
        end

        local valor_lower = string.lower(valor)
        local chave_existente = database_find_by_value(valor_lower)

        if chave_existente ~= nil and chave_existente ~= chave then
            return false, "E-mail já cadastrado na chave '" .. chave_existente .. "'"
        end

        return true, valor_lower
    end,

    select = function(valor)
        return true, valor
    end

}