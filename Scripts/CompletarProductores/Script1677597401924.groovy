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

//
WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Intermediarios')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  I_7457cf/select_ComisinfijaEstructuradegestinNoPoseeComisionSegntabla'), 
    '1', true)

WebUI.click(findTestObject('Page_Tratamiento de plizas (Company ABC)  Intermediarios - Productores/btn_Convenio de cobranza_Add'))

WebUI.switchToWindowTitle('')

WebUI.setText(findTestObject('Page_Productores_Intermediarios/input_Cdigo_valIntermed'), '22300022')

WebUI.click(findTestObject('Page_Productores_Intermediarios/label_Intermediario'))

WebUI.setText(findTestObject('Page_Productores_Intermediarios/input_PorcentajeDeParticipacin'), '100')

WebUI.click(findTestObject('Page_Productores_Intermediarios/label_Intermediario'))

WebUI.click(findTestObject('Page_Productores_Intermediarios/btn_Continuar_cmdAccept'))

//VER SI SE CIERRA
WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Intermediarios')

WebUI.click(findTestObject('Page_Tratamiento de plizas (Company ABC)  Intermediarios - Productores/btnCheckA'))

