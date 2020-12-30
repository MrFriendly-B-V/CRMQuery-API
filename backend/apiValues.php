<?php
$returnValues = [
    'emailAddress',
    'name',
    'createdByName',
    'firstName',
    'lastName',
    'producten',
    'description',
    'phoneNumber',
    'relatieType',
    'relatieNaam'
];

$accountTypeValues = [
    'Ignore Filter',
    'Klant',
    'Overheid',
    'Investeerder',
    'Partner',
    'Reseller',
    'Leverancier',
    'Utility Company'
];

$accountLocationType = [
    'Ignore Filter',
    'Bar/Kroeg',
    'Restaurant',
    'Discotheek/Club',
    'Sport',
    'Museum, Tentoonstelling',
    'Theater',
    'Overheids/bedrijfs gebouw'
];

$accountProductValues = [
    'Ignore Filter',
    'Urinoir: Keramisch (Falcon)',
    'Urinoir: MRF Kunstof',
    'Media Player',
    'Reinigingsmiddelen (EnzyForce)',
    'DanDryer',
    'Luchtreiniger',
    'PVC Wand',
    'Media (Ads)',
    'Filter (Falcon Droog)',
    'Filter (Falcon Nat)',
    'Filter (MRF)',
    'Filter (Geberit Droog)'
];

$provinceVisitAddress = [
    'Ignore Filter',
    'Groningen',
    'Drenthe',
    'Overijssel',
    'Utrecht',
    'Noord-Holland',
    'Zuid-Holland',
    'Zeeland',
    'Noord-Brabant',
    'Limburg (Nederland)',
    'Antwerpen',
    'Limburg (Belgie)',
    'Oost-Vlaanderen',
    'Vlaams-Brabant',
    'West-Vlaanderen',
    'Henegouwen',
    'Luik',
    'Luxemburg (Belgie)',
    'Namen',
    'Waals-Brabant'
];

$contactRoleValues = [
    'Ignore Filter',
    'Directeur/Eigenaar',
    'Bedrijfsleider',
    'Commercieel',
    'Facilitair',
    'Inkoop',
    'Administratie',
    'Technish/IT',
    'Marketing'
];

$response = [
    'returnValues' => $returnValues,
    'accountTypeValues' => $accountTypeValues,
    'accountProductValues' => $accountProductValues,
    'contactRoleValues' => $contactRoleValues,
    'provinceVisitAddress' => $provinceVisitAddress,
    'accountLocationType' => $accountLocationType
];

echo json_encode($response);
?>