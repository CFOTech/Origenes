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

not_run: WebUI.navigateToUrl('http://ori-dtp-vtimeos.retiro.ar:8083/VTimeNet/Common/secWHeader.aspx?sCodispl=CA001&sProject=PolicySeq&sModule=Policy&sConfig=InSequence&bMenu=1')

not_run: WebUI.delay(5)

not_run: WebUI.refresh()

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  A_4bf224/a_Contratante'))

WebUI.switchToWindowTitle('')

WebUI.setText(findTestObject('Object Repository/Page_/input_Cdigo del cliente_tctCode (1)'), '42933160')

WebUI.click(findTestObject('Page_/divPopUp'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Page_/img_Razn Social a imprimir_cmdAccept'))

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Asegurables')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  A_4bf224/a_Ejecutivocomercial'))

WebUI.switchToWindowTitle('')

WebUI.setText(findTestObject('Object Repository/Page_/input_Cdigo del cliente_tctCode (1)'), '21132149')

WebUI.click(findTestObject('Page_/divPopUp'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Page_/img_Razn Social a imprimir_cmdAccept'))

WebUI.switchToWindowTitle('Tratamiento de pólizas (Company ABC) / Asegurables')

WebUI.click(findTestObject('Object Repository/Page_Tratamiento de plizas (Company ABC)  A_4bf224/a'))

WebUI.waitForPageLoad(10)

