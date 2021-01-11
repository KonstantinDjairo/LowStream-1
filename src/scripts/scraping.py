import requests
import json
import warnings
from urllib.parse import quote
import sys, os
import traceback
from time import sleep
import base64
#Anti-Warning, essas desgraça só serve pra poluir
#o terminal com coisa inutil.
warnings.filterwarnings('ignore')
# Requests é uma biblioteca HTTP para Psys.pathython simples e elegante, feita para seres humanos. 
Session = requests.session()
# O cabeçalho de requisição User-Agent é uma cadeia de caracteres característica que permite servidores e pares de rede identificar a aplicação,
#  sistema operacional, fornecedor, e/ou versão do agente de usuário requisitante.
Session.headers = { #Headers pra evitar problema com o site maldito.
    "User-Agent": "Mozilla/5.0 (Linux; Android 10; SM-A307GT) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.198 Mobile Safari/537.36 OPR/61.1.3076.56625",
    "sec-fetch-site": "same-origin",
    "sec-fetch-mode": "cors",
    "sec-fetch-dest": "empty"
}


#Func rápida pra usar um urlsafe "real'
# a na função é uma bool
quotSafe = lambda a: quote(a, safe = "")
# Função que resebe os paramentros 
# funação com parametros url = "https://5.orezraey.workers.dev/0:/Animes/"
# função recursiva, ela vai rodando, se 5 erros ela para 
def RequestErros(url, data, delay = 1, max_err = 10, err = 0):
    """
        Essa func maldita serve pra mandar dados ao servidor
        e em caso de erro, tentar novamente até dar certo(Ou chegar ao limite)
    
        max_err é, como o nome diz, o limite de erros no server.
        err é pra saber quantos erros já deu na request.
        Já o resto é MAIS óbvio
    """
    # pegando a variavel global 
    global Session
    # sleep com um paramentro
    sleep(delay)
    #Verify False pra evitar erros de ssl bosta
    #Em compensação dá vários warns, mas já silenciamos eles request 
    request  = None
    # erroDelay recebe o err, no momento err = 0
    errDelay = err #Delay pro sleep em caso de erro bruto
    # errSTring
    errStr = ""
    # teste de erros 
    # O try/except serve para tratamento de exceções.
    # Exceção é quando uma coisa não ocorre da forma como foi planejada. 
    # Pode-se dizer que os programas tem um fluxo normal, e um fluxo excepcional que são as exceções.
    # A sintaxe é
    # try:
    #     código a tentar
    # except AlgumaExcecao:
    #      código a executar no caso da exceção
    # else:
    #     código a executar caso não ocorra exceção em try
    # finally:
    #      código que é executado sempre, independente de haver uma exceção em andamento ou não
    try:
        # request recebe session com o medoto post, passano a url com + paramentros 
        request  = Session.post(url, data = data, verify = False, timeout = 10)
        # response.status_code ou request.status_code retorna um número que indica o status (200 é OK, 404 não encontrado). 
        # As solicitações Python 
        # geralmente são usadas para buscar o conteúdo de um determinado URI de recurso. Sempre que fazemos uma solicitação
        # a um URI especificado por meio do Python, ele retorna um objeto de resposta. Agora, esse objeto de resposta seria 
        # usado para acessar certos recursos, como conteúdo, cabeçalhos, etc. Este artigo gira em torno de como verificar 
        # o response.status_code de um objeto de resposta. Para saber mais sobre os códigos de status para HTTP, visite -
        # Códigos de status HTTP | Respostas bem-sucedidas e códigos de status HTTP | Respostas
        # status recebe a request com metodo status_code 
        status = request .status_code
        # verifica se status é retorna um numero diferente de 200, se sim true
        if status != 200:
            # A raise instrução permite que o programador force a ocorrência de uma exceção especificada
            # O único argumento para raiseindica a exceção a ser levantada. Deve ser uma instância de 
            # exceção ou uma classe de exceção (uma classe que deriva de Exception).
            raise Exception("hm")
        # request recebe request com metodo json()
        request = request.json()
        if "error" in request ["data"]:
            #Delay pra Rate-Limit
            errDelay = 180+err
            # errString
            errStr = "Limite de requisições"
    # se nao, faça isso 
    # except AlgumaExcecao:
    #      código a executar no caso da exceção
    except Exception as e:
        # request é nome?
        if request  is None:
            #Erro "conn refused" ou coisa pior
            #Pode ser causado por ativar vpn do nada no processo
            #Ou pelo server "cair"
            errStr = "SERVER TOROU FOI TUDO IRMÃO, HAJA PACIÊNCIA"
            # caso for errDelay recebe os segundos + err, erro é um paramentro da função
            errDelay = 120 + err
        else:
            #Delay pra erro 500(Erro do textão e outros)
            errDelay = 10+err
            # pra ser exibida em algum print 
            errStr = "StatusCode 500, server lixo"
    # se errDelay for diferente de err
    if errDelay != err:
        # vai exibir os seguintes erros 
        print(f"\nErro na request, Num {err+1}")
        print(f"Erro: {errStr}")
        # exibir a url onde ta o deu o erro 
        print(f"URL: {url}")
        # verifica se erro é igual o maximo de erro, se for o bagulho zoa kjjkjkj
        if err == max_err: #Limite eh foda firma
            print("Puta q pariu irmão, servidor tá de putaria")
            print(f"Se deu {err} erros, o jeito é desistir msmKKKKKKK")
            # aumento permite lançar uma exceção a qualquer momento. assert permite que você verifique
            #  se uma determinada condição é atendida e lança uma exceção se não for. Na cláusula try, 
            # todas as instruções são executadas até que uma exceção seja 
            # encontrada. exceto é usado para capturar e tratar as exceções encontradas na cláusula try.
            raise Exception("Zuo foi tudo")
        # mostra os segundos 
        print(f"Delay de {errDelay} segundos só de zoa")
        # da o delay
        sleep(errDelay)
        # volta
        print("Beleza tentando dnv")
        #Recursividade né pora, tentar dnv 
        #NÃO MEXA NO "err+1" TALKEI
        request  = RequestErros(url, data, delay, max_err, err+1)
    # retorna a request 
    return request 



# path = caminho
# fullData com o paramentro path    
def fullData(path=""):
    """
        Tenta pegar todos os dados possíveis, como se estivesse
        descendo e atualizando a página.
        Param path é meio óbvio: é a "pasta" pra ir.
    """
    #O server buga se não tiver "/" no final do path, então
    #o ternário corrige isso
    path += "/" if path != "" else "" 
    # url a ser pegada 
    url = "https://5.orezraey.workers.dev/0:/Animes/"
    #Dados de sempre pra enviar pro maldito servidor
    initData = {
        "q": "", #Inutil
        "password": None, #Inutil
        "page_token": None, # IMPORTANTISSIMO
        "page_index": 0 #Mt importante tbm
    }
    #Json.dumps pra transformar a Dict em Json.
    postData = json.dumps(initData) 
    dados = [] #Total de dados recolhidos
    #Diminui o delay se estiver pegando dados da page principal.
    delay = 0.1 if path == "" else 1
    # loop ate um break
    while True:
        # resquest recebe a funçao RequestErros com os parametros 
        request  = RequestErros(url+path, data = postData, delay = delay)
        #Adiciona á lista de dados recolhidos
        dados +=  request ["data"]["files"] 
        #Vê se ainda tem page pela frente
        if  request ["nextPageToken"] is not None:
            # initdata recebe rquest, as 2 sao uma lista 
            initData["page_token"] =  request ["nextPageToken"]
            #Ternário gigante e feio, que traduzindo faz a "page_index"
            #ser maior que a atual, mas se a atual for nula, vira 1
            initData["page_index"] =  request ["curPageIndex"]+1 if  request ["curPageIndex"] is not None else 1
            #Dict2Json dnv            
            postData = json.dumps(initData)
        # fim    
        else:
            #Pegou tudo garaiKKKKK
            break 
    # retotna os dados 
    return dados
    

# função que pega o link de Download/video dos eps
def DownloadUrl(nome, ep, fonte = False):
    # url principal 
    url = "https://5.orezraey.workers.dev/0:/Animes"
    #Cada quotSafe é uma transformação em urlsafe
    #Já o ternário é pra colocar a fonte direito(se existir)
    # vsf kkkkkkk
    # basicamente é, uma string sendo modificada com o metodo f do python
    # se fonte for verdadeira? ????????????????? 
    return f"{url}/{quotSafe(nome)}/{quotSafe(fonte)+'/' if fonte else ''}{quotSafe(ep['name'])}"
    

# add o link do player 
def PlayerUrl(nome, ep, fonte = False):
    # url principal 
    url = "https://5.orezraey.workers.dev/0:video"
    #Não pode usar o treco de URLSAFE, então usei replace msm
    fonte = fonte.replace("/","%2F")+"/" if fonte else ""
    nome = nome.replace("/","%2F")
    ep = ep['name'].replace("/","%2F")
    #Encode pra poder usar no B64
    padrao1 = f"/0:/Animes/{nome}/{fonte}{ep}".encode()
    #E pra vingar, um decode pra usar na F-String
    # B64_STR recebe a string que do player
    # exemeplo: https://5.orezraey.workers.dev/0:video/LzA6L0FuaW1lcy8xMDAtb........
    B64_STR = base64.urlsafe_b64encode(padrao1).decode()
    # retorna com os dados pronto 
    return f"{url}/{B64_STR}"
    


# "função" principal 
if __name__ == "__main__": #Preguiça de fazer func

    #Verifica se o arquivo lista.json existe.
    #Se existir, carrega ele e acelera o code.
    #Se não existir, 6coleta os dados e cria um, pra acelerar uma próxima rodada
    """if os.path.exists("lista.json"):
        Animes = json.load(open("lista.json"))
    else:
        Animes = fullData()
        #Indent 4 so pra ficar bonito de ler, mas é INÚTIL
        #e cria arquivos GRANDES. recomendo remover na prática
        json.dump(Animes, open("lista.json","w+"),indent = 4)"""
    # anime recebe a função FullData
    Animes = fullData()
    # verifca de  se o arquivo Animes.json existe
    if os.path.exists("Animes.json"):
        # se exitir abre o msm
        dados = json.load(open("Animes.json"))
    else:
        # se nao existir faz um dicionario 
        dados = {}
    # url principal  
    url = "https://5.orezraey.workers.dev/0:/Animes/"
    #NUNCA TAQUE TUDO NUM TRY-EXCEPT/CATCH
    #Eu to totalmente fodase pro code, tá terrivel, logo to usando
    #MAS NÃO RECOMENDO
    try:
        # loop anime emm Animes que recebeu a função FullData 
        for anime in Animes:
            # nome do anime 
            nome = anime["name"]
            #Acelera o code, em caso de erros no passado.
            if nome in dados:
                continue
            # axibe o nome do anime
            print(f"\nAnime: {nome}")
            # verifaica se tem text em anime["mimeType"]
            if "text" in anime["mimeType"]:
                # se tiver para o loop
                break
            # eps recebe a função FullData com o quotSafe(nome)
            #Dados do anime.
            eps = fullData(quotSafe(nome)) 
            # lista fontes
            fontes = []
            #Template pra facilitar um futuro usuário do code.
            default = {"padrao":True, "fonte": "A Firma", "eps": []}
            # lopp ep em eps
            for ep in eps:
                # epname recebe nome do ep
                epName = ep["name"]
                # exibe o nome dos eps
                print(f"\nEP: {epName}")
                # exibe o mimeType
                print(f"MimeType: {ep['mimeType']}")
                # verifca se .text em ep["mimeType"]
                if(".text" in ep["mimeType"]):
                    break #Textos ficam no final da page, logo é uma boa sair
                #Verifica se é uma pasta
                if "folder" in ep["mimeType"]: 
                    custom = {"padrao":False, "fonte": epName, "eps": []}
                    #Enfia urlsafe no nome do anime e no nome da pasta
                    tmpath = quotSafe(nome)+"/"+quotSafe(epName)
                    print(f"{nome}/{epName} = {tmpath}")
                    dat2 = fullData(tmpath) #Dados da pasta

                    for realEp in dat2:
                        del realEp["id"] 
                        if not "text" in realEp["mimeType"]:
                            #Urls úteis
                            realEp["downloadUrl"] = DownloadUrl(nome, realEp, epName)
                            realEp["playerUrl"] = PlayerUrl(nome, realEp, epName)
                            print(f"Ep REAL: {realEp['name']}")
                            #Enfia os eps na lista da fonte.
                            custom["eps"].append(realEp)
                    #E finalmente, adiciona na lista de fontes.
                    fontes.append(custom)
                else:
                    #caso não seja pasta, só adiciona á template msm
                    del ep["id"]
                    if not "text" in ep["mimeType"]:
                        #Urls úteis
                        ep["downloadUrl"] = DownloadUrl(nome, ep)
                        ep["playerUrl"] = PlayerUrl(nome, ep)
                        default["eps"].append(ep)
            if len(default["eps"]) != 0:
                #Adiciona a template ás fontes se tiver sido usada.
                fontes.append(default)
            #Finalmente ajeita as coisas e enfia na lista de animes.
            dados[nome] = fontes
    except:
        #Salvar nos erros pra não ter que voltar do começo depois
        print(traceback.format_exc())
        # Usamos o dump e o dumps para transformar um objeto do Python 
        # (listas, dicionários, tuplas) nos formato JSON (serializar)
        # abrindo o arquivo + json + dump, dando 4 espaços de indentação   
        json.dump(dados, open("Animes.json","w+"))
        # sys.exit () levanta uma exceção SystemExit que você provavelmente está assumindo como algum erro. 
        # Se você deseja que seu programa não acione SystemExit, mas retorne normalmente, você pode agrupar
        # sua funcionalidade em uma função e retornar de lugares onde está planejando usar sys.exit
        sys.exit()
    # se nao faça o bagulho principal
    else:
        # Usamos o dump e o dumps para transformar um objeto do Python 
        # (listas, dicionários, tuplas) nos formato JSON (serializar)
        # abrindo o arquivo + json + dump, dando 4 espaços de indentação   
        json.dump(dados, open("Animes.json","w+"))
