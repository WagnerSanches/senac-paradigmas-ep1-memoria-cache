return {
    prefixo = "cpf_",
    insert = function(chave, valor)
        local resultado = database_select(chave)
        print("Lua recebeu do Rust: " .. resultado)
        return "ok"
    end
}