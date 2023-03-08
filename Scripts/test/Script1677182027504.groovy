import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase

import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

def nroPoliza = '901209312/4'

nroPoliza = nroPoliza.substring(0, nroPoliza.lastIndexOf('/'))

println nroPoliza