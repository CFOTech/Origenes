import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.closeWindowTitle('VisualTIME 7.0')

WebUI.delay(2)

WebUI.switchToWindowTitle('Interfaz de Operaciones Emitidas (Company ABC)')

WebUI.click(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/btnGuardarCheck'))

WebUI.setText(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/inpFechaInicial'), '06/03/2023')

WebUI.setText(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/inpFechaFinal'), '05/03/2024')

WebUI.selectOptionByValue(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/selectRamo'), '9', false)

WebUI.setText(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/inpProducto'), '9100')

WebUI.setText(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/inpPoliza'), GlobalVariable.nroPoliza)

WebUI.click(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/btnFinalizar'))

WebUI.delay(3)

WebUI.acceptAlert()

WebUI.switchToWindowTitle('Interfaz de Operaciones Emitidas (Company ABC)')

WebUI.click(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/btnIrAProcBatch'))

WebUI.switchToWindowTitle('Estado de procesos batch (Company ABC) / Estado de procesos batch')

WebUI.click(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/btnActivarProceso'))

WebUI.switchToWindowTitle('')

WebUI.check(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/chkActivar'))

WebUI.click(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/lblBatch'))

WebUI.click(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/img_Continuar_cmdAccept'))

WebUI.switchToWindowTitle('Estado de procesos batch (Company ABC) / Estado de procesos batch')

WebUI.delay(3)

WebUI.refresh()

WebUI.click(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/btnEstado'))

WebUI.switchToWindowTitle('')

def nombreArchivo = WebUI.getText(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/lblArchivo'))

WebUI.click(findTestObject('Page_Interfaz de Operaciones Emitidas (Company ABC)/btnVerArchivo'))

return nombreArchivo

