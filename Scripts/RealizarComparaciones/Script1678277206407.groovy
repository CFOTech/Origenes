import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

//Renombrado de archivo
File file = new File(('C:\\Users\\Julian\\Downloads\\' + nombreArchivo) + '.xls')

File file2 = new File(('C:\\Users\\Julian\\Downloads\\' + nombreArchivo) + '.html')

file.renameTo(file2)

//Abrir el excel como una pagina web
WebUI.openBrowser(('C:\\Users\\Julian\\Downloads\\' + nombreArchivo) + '.html')

WebUI.scrollToElement(findTestObject('Reporte Poliza/td_PrimaTarifa'), 3)

//Obtener variable de interes
def primaReporte = WebUI.getText(findTestObject('Reporte Poliza/td_PrimaTarifa'))

KeywordUtil.logInfo('Prima obtenida: ' + primaReporte)
KeywordUtil.logInfo('Prima esperada: ' + primaReporte)

def premioReporte = WebUI.getText(findTestObject('Reporte Poliza/td_Premio'))

KeywordUtil.logInfo('Premio obtenido: ' + primaEsperada)

KeywordUtil.logInfo('Premio esperado: ' + premioEsperado)

WebUI.closeBrowser()

assert (primaReporte == primaEsperada) || (premioReporte == premioEsperado)

