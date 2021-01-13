import requests
import json
import os, sys
from time import sleep
import traceback
from mal import AnimeSearch
from mal import Anime


# função faz tudo 
def AnimesSynopsis():
    lista_animes = []
    id = 1
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
    print(">>>>>>>>>>>>>> aguarde <<<<<<<<<<<<<")
    print("primeira request demora mesmo")
    # for na lista  de animes do json
    for  nomes_animes in dados_json.keys():
        # tente pegar os dados
        try:
            # faz a requests com o paramentro nome do animes que veio do json
            search = AnimeSearch(nomes_animes)
            # usa o paramentro pra fazer outra request
            anime = Anime(search.results[0].mal_id)
            # hmmmmmmmmmmmm
            print("-____________________________________________________________________________________-")
            print(id)
            print(f"IDs: {anime.mal_id} == {search.results[0].mal_id}")
            print("nome: ", anime.title)
            print("sinopse: ", anime.synopsis)
            print("rank: ", anime.rank)
            print("popularidade: ", anime.popularity)
            print("genero: ", anime.genres)
            print("studio: ", anime.studios)
            print("premiações: ", anime.premiered)
            print("score: ", anime.scored_by)
            # adiciona na lista
            lista_animes.append({"id": id, "title": anime.title, "synopsis": anime.synopsis,
                "rank": anime.rank, "popularity": anime.popularity, "genres": anime.genres,
                "studios": anime.studios, "premiered": anime.premiered, "scored_by": anime.scored_by })
            # coloca os id
            id += 1
        # torça pra nao da erros
        except:
            # hmm
            erros += 1
            print(traceback.format_exc())
            print(f" erros: {erros}")
            print("puta merda, ou é minha net de traquinagem ou alguma outra merda deu, le o erro ai lkjkhj")
            print("espera 10 segundos, se nao vai que")
            sleep(5)
    # retorna os dados prontos
    return lista_animes


# praks bro
if __name__ == "__main__":
    # pega os dados da função que faz os trabalho todo
    dados = AnimesSynopsis()
    print("tudo pronto, agora vai salvar")
    # salvo no json
    with open("AnimesDesc.json", "w") as json_file:
        # faz virar objeto do python
        json.dump(dados, json_file, indent = 4)

# feito por Mateus Rodrigues         