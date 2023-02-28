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

not_run: WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Coberturas')

WebUI.uncheck(findTestObject('Page_Tratamiento de plizas (Company ABC)  Coberturas/input_Noaplica_Sel'))

WebUI.delay(3)

WebUI.uncheck(findTestObject('Page_Tratamiento de plizas (Company ABC)  Coberturas/input_Noaplica_Sel (1)'))

WebUI.click(findTestObject('Page_Tratamiento de plizas (Company ABC)  Coberturas/a_GASTOSREEMPLAZOLLAVES'))

WebUI.switchToWindowTitle('')

WebUI.clearText(findTestObject('Page_Cobertura/input_Suma asegurada_tcnCapital'))

WebUI.setText(findTestObject('Page_Cobertura/input_Suma asegurada_tcnCapital'), '800')

WebUI.click(findTestObject('Page_Cobertura/table_Mdulovar mstrDoSubmit'))

WebUI.click(findTestObject('Page_Cobertura/img_Continuar_cmdAccept'))

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Coberturas')

WebUI.click(findTestObject('Page_Tratamiento de plizas (Company ABC)  Coberturas/a_GASTOSREEMPLAZODOCUMENTOS'))

WebUI.switchToWindowTitle('')

WebUI.clearText(findTestObject('Page_Cobertura/input_Suma asegurada_tcnCapital'))

WebUI.setText(findTestObject('Page_Cobertura/input_Suma asegurada_tcnCapital'), '800')

WebUI.click(findTestObject('Page_Cobertura/table_Mdulovar mstrDoSubmit'))

WebUI.click(findTestObject('Page_Cobertura/img_Continuar_cmdAccept'))

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Coberturas')

WebUI.click(findTestObject('Page_Tratamiento de plizas (Company ABC)  Coberturas/a_ROBOBOLSOCARTERAMALETIN'))

WebUI.switchToWindowTitle('')

WebUI.clearText(findTestObject('Page_Cobertura/input_Suma asegurada_tcnCapital'))

WebUI.setText(findTestObject('Page_Cobertura/input_Suma asegurada_tcnCapital'), '6900')

WebUI.click(findTestObject('Page_Cobertura/table_Mdulovar mstrDoSubmit'))

WebUI.click(findTestObject('Page_Cobertura/img_Continuar_cmdAccept'))

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Coberturas')

WebUI.uncheck(findTestObject('Page_Tratamiento de plizas (Company ABC)  Coberturas/input_Noaplica_Sel'))

WebUI.delay(3)

WebUI.uncheck(findTestObject('Page_Tratamiento de plizas (Company ABC)  Coberturas/input_Noaplica_Sel (1)'))

WebUI.check(findTestObject('Page_Tratamiento de plizas (Company ABC)  Coberturas/input_Noaplica_Sel (1) - Copy'))

WebUI.click(findTestObject('Page_Tratamiento de plizas (Company ABC)  Coberturas de una pliza/btnCheckA'))

