import requests
import json
import os, sys
from time import sleep
import traceback
from mal import AnimeSearch

# função faz tudo 
def AnimesSynopsis():
    lista_animes = []
    id = 0
    erros = 0
    # tenta entrar no json pra pegar os animes - nomes 
    try:
        # arquivo json
        arquivo_json = open('Animes.json', 'r')
        dados_json = json.load(arquivo_json)
    # caso nao conseguir sai fora 
    except:
        print(traceback.format_exc())
        print("verifica o arquivo ou nome e saia daqui")
        # sair
        sys.exit()
    # for na lista  de animes do json
    for  nomes_animes in dados_json.keys():
        # tente pegar os dados
        try:
            # faz a requests com o paramentro nome do animes que veio do json
            search = AnimeSearch(nomes_animes)
            # printa o resultado, pra ver so
            print(search.results[0].title)
            # printa a descrição
            print(search.results[0].synopsis) # Get title of first result
            lista_animes.append({"id": id, "title": search.results[0].title, "synopsis": search.results[0].synopsis})
            # coloca os id
            id += 1
        # tornça pra nao da erros
        except:
            # hmm
            erros += 1
            print(traceback.format_exc())
            print(f" erros: {erros}")
    # retorna os dados prontos
    return lista_animes


# pai é chato
if __name__ == "__main__":
    # pega os dados da função que faz os trabalho todo
    dados = AnimesSynopsis()
    print("hmmmmmmmmmmm")
    # salvo no json
    with open("AnimesDesc.json", "w") as json_file:
        # faz virar objeto do python
        json.dump(dados, json_file, indent=4)

# feito por Mateus Rodrigues         