function populateForm(): void {
    var loadingIcon = document.getElementById('loader');
    loadingIcon.style.visibility = 'hidden';

    //Get the API filter values and populate the Select objects
    $.when(getApiValues().done(function(e) {
        var responseJson = JSON.parse(e);

        //What values do we return to the user (e.g Email, firstName)
        var returnValues: string[] = responseJson.returnValues;
        var returnValuesSelect: HTMLElement = document.getElementById('returnType');
        for(var i = 0; i < returnValues.length; i++) {
            var value = returnValues[i];
            returnValuesSelect.append(createOptionDOM(value));
        }

        //What products does the account have from us
        var accountProductValues: string[] = responseJson.accountProductValues;
        var accountProductSelect: HTMLElement = document.getElementById('accountProductQuery');
        for(var i = 0; i < accountProductValues.length; i++) {
            var value = accountProductValues[i];
            accountProductSelect.append(createOptionDOM(value));
        }

        //Type of the location of the account (e.g Bar, Restaurant)
        var accountLocationTypeValues: string[] = responseJson.accountLocationType;
        var accountLocationTypeField: HTMLElement = document.getElementById('accountLocationType');
        for(var i = 0; i < accountLocationTypeValues.length; i++) {
            var value = accountLocationTypeValues[i];
            accountLocationTypeField.append(createOptionDOM(value));
        }

        //Type of the account (e.g Government, Supplier)
        var accountTypeValues: string[] = responseJson.accountTypeValues;
        var accountTypeField: HTMLElement = document.getElementById('accountTypeQuery');
        for(var i = 0; i < accountTypeValues.length; i++) {
            var value = accountTypeValues[i];
            accountTypeField.append(createOptionDOM(value));
        }

        //Province where the visit address is located (e.g Groningen, Friesland)
        var provinceVisitAddressValues: string[] = responseJson.provinceVisitAddress;
        var provinceVisitAddressField: HTMLElement = document.getElementById('provinceVisitAddress');
        for(var i = 0; i < provinceVisitAddressValues.length; i++) {
            var value = provinceVisitAddressValues[i];
            provinceVisitAddressField.append(createOptionDOM(value));
        }

        //ContactRole
        var contactRoleValues: string[] = responseJson.contactRoleValues;
        var contactRoleSelect: HTMLElement = document.getElementById('contactQuerySelector');
        for(var i = 0; i < contactRoleValues.length; i++) {
            var value = contactRoleValues[i];
            contactRoleSelect.append(createOptionDOM(value));
        }

        //Lastly, set the DOM Option objects as chosen
        //JQuery Chosen library
        (<any>$(".chosen-select")).chosen();

        (<any>$(".tagging")).select2({
            tags: true,
            tokenSeparators: [',', ' ']
        });
    }));

    $.getJSON('backend/known_cities.json', function(responseJson) {

        //City where the visit address is located
        //Note: This can be empty, as we fill the list based on what users enter
        //e.g Amsterdam, Rotterdam
        var knownCities: string[] = responseJson.known_cities;
        var cityVisitAddressField: HTMLElement = document.getElementById('cityVisitAddress');
        cityVisitAddressField.append(createOptionDOM("Ignore Filter"));
        for(var i = 0; i < knownCities.length; i++) {
            var value = knownCities[i];
            cityVisitAddressField.append(createOptionDOM(value));
        }
    });
}

function createOptionDOM(value: string) {
    var option = document.createElement('option');
    option.value = value;
    option.innerHTML = value;

    return option;
}

//Function is called onClicked for the submit button
function queryApi(): void {
    var loadingIcon = document.getElementById('loader');
    loadingIcon.style.visibility = 'visible';

    var returnTypes: string[] = <string[]> $('#returnType').val();
    var accountProductQuery: string[] = <string[]> $('#accountProductQuery').val();
    var accountTypeQuery: string[] = <string[]> $('#accountTypeQuery').val();
    var accountLocationTypeQuery: string[] = <string[]> $('#accountLocationType').val();
    var provinceVisitAddress: string[] = <string[]> $('#provinceVisitAddress').val();
    var cityVisitAddress: string[] = <string[]> $('#cityVisitAddress').val();
    var contactQuery: string[] = <string[]> $('#contactQuerySelector').val();

    $.ajax({
        url: 'backend/api.php',
        type: 'post',
        data: {
            'accountProductQuery': accountProductQuery.join(),
            'accountTypeQuery': accountTypeQuery.join(),
            'accountLocationTypeQuery': accountLocationTypeQuery.join(),
            'provinceVisitAddress': provinceVisitAddress.join(),
            'cityVisitAddress': cityVisitAddress.join(),
            'contactQuery': contactQuery.join()
        },
        success: function(e: string) {

            //Get the results Table and clear it
            var resultTable: HTMLTableElement = <HTMLTableElement> document.getElementById('resultTable');
            resultTable.innerHTML = "";

            //Parse the response from the backend to JSON
            var jsonResult;
            try {
                jsonResult = JSON.parse(e);
            } catch (ex) {
                console.log(e);
                loadingIcon.style.visibility = 'hidden';
                alert("An error occured!");
                return;
            }

            var rowIndex = 0;

            //Set the table headers
            var tableHeaderRow: HTMLElement = resultTable.insertRow(rowIndex);
            rowIndex++;

            var cellIndex = 0;
            for(var j = 0; j < returnTypes.length; j++) {
                if(returnTypes[j] == 'producten') continue;
                if(returnTypes[j] == 'relatieType') continue;
                if(returnTypes[j] == 'relatieNaam') continue;

                var returnType: string = returnTypes[cellIndex];
                var header: HTMLElement = document.createElement('th');
                header.innerHTML = returnType;

                tableHeaderRow.append(header);
                cellIndex++;
            }

            //Add a header for the 'Producten' column
            if(returnTypes.includes('producten')) {
                var productenHeader = document.createElement('th');
                productenHeader.innerHTML = "Producten";
                tableHeaderRow.append(productenHeader);
            }

            if(returnTypes.includes('relatieType')) {
                var relatieTypeHeader = document.createElement('th');
                relatieTypeHeader.innerHTML = "Relatie Type";
                tableHeaderRow.append(relatieTypeHeader);
            }
            
            if(returnTypes.includes('relatieNaam')) {
                var relatieTypeHeader = document.createElement('th');
                relatieTypeHeader.innerHTML = "Relatie naam";
                tableHeaderRow.append(relatieTypeHeader);
            }

            //Array to keep track which Contacts we've already had
            var contactIds = [];

            //Iterate over all provided accounts (leads)
            for(var i = 0; i < jsonResult.length; i++) {
                var item = jsonResult[i];
                var accountProducten = item.accountProducten;
                var relatieType = item.relatieType;
                var relatieNaam = item.relatieNaam;
                var contactList = item.contacts.list;

                //Iterate over every contact in the current account
                for(var j = 0; j < contactList.length; j++) {
                    var contact = contactList[j];

                    //Create a row for this contact
                    var contactRow = resultTable.insertRow(rowIndex);
                    rowIndex++;
                    
                    //Check if the contact ID is already in the table,
                    //if so we want to skip it
                    if(contactIds.includes(contact['id'])) continue;

                    //We don't have the current Contact in the table just yet,
                    //But we will have, so add the Contact's id to the array
                    contactIds.push(contact['id']);

                    //Used to keep track what the last cell is, because we
                    //need to add after that later
                    var rowSize = 0;

                    //Iterate over the return types we specified,
                    //and add those return types to the results table
                    for(var k = 0; k < returnTypes.length; k++) {
                        if(returnTypes[k] == 'producten') continue;
                        if(returnTypes[k] == 'relatieType') continue;
                        if(returnTypes[k] == 'relatieNaam') continue;

                        var cell = contactRow.insertCell(rowSize);
                        cell.innerHTML = contact[returnTypes[k]];
                        cell.classList.add("resultCell");

                        rowSize++;
                    }

                    //Besides what the account gives us, we also want to tell the user what
                    //the contact's Account uses from us
                    if(returnTypes.includes('producten')) {
                        var productenCell = contactRow.insertCell(rowSize);
                        productenCell.innerHTML = accountProducten;
                        productenCell.classList.add("resultCell");
                    }

                    if(returnTypes.includes('relatieType')) {
                        var relatieTypeCell = contactRow.insertCell(rowSize);
                        relatieTypeCell.innerHTML = relatieType;
                        relatieTypeCell.classList.add("resultCell");
                    }

                    if(returnTypes.includes('relatieNaam')) {
                        var relatieNaamCell = contactRow.insertCell(rowSize);
                        relatieNaamCell.innerHTML = relatieNaam;
                        relatieNaamCell.classList.add("resultCell");
                    }
                }
            }

            loadingIcon.style.visibility = 'hidden';
        }
    });
}

//Function used to retrieve all possible filter values for the API
function getApiValues(): JQuery.jqXHR {
    return $.ajax({
        url: 'backend/apiValues.php',
        type: 'get'
    });
}

// Quick and simple export target #table_id into a csv
function download_table_as_csv(table_id, separator = ',') {
    // Select rows from table_id
    var rows = document.querySelectorAll('table#' + table_id + ' tr');
    // Construct csv
    var csv = [];
    for (var i = 0; i < rows.length; i++) {
        var row = [], cols: any = rows[i].querySelectorAll('td, th');
        for (var j = 0; j < cols.length; j++) {
            // Clean innertext to remove multiple spaces and jumpline (break csv)
            var data = cols[j].innerText.replace(/(\r\n|\n|\r)/gm, '').replace(/(\s\s)/gm, ' ')
            // Escape double-quote with double-double-quote (see https://stackoverflow.com/questions/17808511/properly-escape-a-double-quote-in-csv)
            data = data.replace(/"/g, '""');
            // Push escaped string
            row.push('"' + data + '"');
        }
        csv.push(row.join(separator));
    }
    var csv_string = csv.join('\n');
    // Download it
    var filename = 'export_' + table_id + '_' + new Date().toLocaleDateString() + '.csv';
    var link = document.createElement('a');
    link.style.display = 'none';
    link.setAttribute('target', '_blank');
    link.setAttribute('href', 'data:text/csv;charset=utf-8,' + encodeURIComponent(csv_string));
    link.setAttribute('download', filename);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
}