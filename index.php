<html lang="en">
    <head>
        <meta charset="utf-8">
		<?php require "../common_include.php"; ?>

        <!--Third party scripts-->
        <script src="https://cdnjs.cloudflare.com/ajax/libs/jquery/3.5.1/jquery.min.js" integrity="sha512-bLT0Qm9VnAYZDflyKcBaQ2gg0hSYNQrJ8RilYldYQ1FxQYoCLtUjuuRuZo+fjqhx/qtq/1itJ0C2ejDxltZVFg==" crossorigin="anonymous"></script>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/chosen/1.8.7/chosen.jquery.min.js" integrity="sha512-rMGGF4wg1R73ehtnxXBt5mbUfN9JUJwbk21KMlnLZDJh7BkPmeovBuddZCENJddHYYMkCh9hPFnPmS9sspki8g==" crossorigin="anonymous"></script>
        <script src="https://cdn.jsdelivr.net/npm/select2@4.1.0-beta.1/dist/js/select2.min.js"></script>
        
        <!--Third party stylesheets-->
        <link href="https://cdn.jsdelivr.net/npm/select2@4.1.0-beta.1/dist/css/select2.min.css" rel="stylesheet" />
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/chosen/1.8.7/chosen.min.css" integrity="sha512-yVvxUQV0QESBt1SyZbNJMAwyKvFTLMyXSyBHDO4BG5t7k/Lw34tyqlSDlKIrIENIzCl+RVUNjmCPG+V/GMesRw==" crossorigin="anonymous" />

        <!--Scripts-->
        <script src="dist/main.js"></script>

        <title> EspoCRM API</title>
    </head>
    <body>
        <!--This webpage does not work without JavaScript enabled-->
        <noscript>
            <p> This page requires JavaScript to be enabled to function.</p>
        </noscript>

        <?php 
        	//Check if the user is authenticated to visit this page
            $from = "https://intern.mrfriendly.nl/crmquery";
            //require "../common_login.php" 
        ?>

        <div class="root">

            <!--Navigation bar-->
			<header>
				<div class="nav-container">
					<a href="https://mrfriendly.nl/"><img alt="logo" class="logo" src="images/logo.png"></a>
					<ul class="navlist">
						<li><a href="https://intern.mrfriendly.nl/dashboard" class="active"> Dashboard </a></li>
						<li><a href="https://mrfriendly.nl"> Website </a></li>
					</ul>
				</div>
            </header>

            <!--Form for filters-->
            <div class="formHolder">
                <form id='queryForm' name='queryForm' target='hiddenIframe' method='post' onsubmit="CRMQuery.queryApi()">
                    <label class="formElement formLabel" for="accountProductQuery"> Afgenomen Producten Relatie</label><br>
                    <select class="formElement Forminput chosen-select" name="accountProductQuery" id="accountProductQuery" multiple required>
                        <option value=""></option>
                        <!--Populated by index.ts#populateForm()-->
                    </select><br>

                    <label class="formElement formLabel" for="accountTypeQuery"> Type Relatie</label><br>
                    <select class="formElement Forminput chosen-select" name="accountTypeQuery" id="accountTypeQuery" multiple required>
                        <option value=""></option>
                        <!--Populated by index.ts#populateForm()-->
                    </select><br>

                    <label class="formElement formLabel" for="accountLocationType"> Soort Locatie</label><br>
                    <select class="formElement Forminput chosen-select" name="accountLocationType" id="accountLocationType" multiple required>
                        <option value=""></option>
                        <!--Populated by index.ts#populateForm()-->
                    </select><br>

                    <label class="formElement formLabel" for="provinceVisitAddress"> Provincie Bezoekadres</label><br>
                    <select class="formElement Forminput chosen-select" name="provinceVisitAddress" id="provinceVisitAddress" multiple required>
                        <option value=""></option>
                        <!--Populated by index.ts#populateForm()-->
                    </select><br>

                    <label class="formElement formLabel" for="cityVisitAddress"> Stad Bezoekadres</label><br>
                    <select class="formElement Forminput tagging" name="cityVisitAddress" id="cityVisitAddress" multiple required>
                        <!--Populated by index.ts#populateForm()-->
                    </select><br>

                    <label class="formElement formLabel" for="contactQuerySelector"> Contact Rol</label><br>
                    <select class="formElement Forminput chosen-select" name="contactQuerySelector" id="contactQuerySelector" multiple required>
                        <option value=""></option>
                        <!--Populated by index.ts#populateForm()-->
                    </select><br>

                    <label class="formElement formLabel" for="returnType"> Return Values </label><br>
                    <select class="formElement Forminput chosen-select" name="returnType" id="returnType" multiple required>
                        <option value=""></option>
                        <!--Populated by index.ts#populateForm()-->
                    </select><br>

                    <input class="formElement submitButton" type="submit"><br>
                </form>
            </div>

            <!--Spinning icon-->
            <div class="loader" id='loader'></div>

            <!--Results from the API-->
            <h2 class="resultLabel"> Result </h2>
            <button id="downloadResultsBtn" class="downloadResultsBtn" onclick="CRMQuery.download_table_as_csv('resultTable', ',');">Download as CSV</button>
            <table id='resultTable'>
                <!--Populated by index.ts#queryApi()-->
            </table>
            <iframe name='hiddenIframe' style="display:none;" width="1px" heigth="1px"></iframe>
        </div>

        <!--Fill in the form options with their values-->
        <script> CRMQuery.populateForm() </script>
    </body>
</html>