# H3c — fontes (substituto do stream "prisões" carData::Arrests)

Recuperado em 2026-10-15. Apenas o esquema foi inspecionado (colunas, dtypes, contagens de linhas,
faixa de datas, conjunto de níveis). NENHUMA taxa de desfecho, calibração ou cruzamento por raça foi
calculado — critérios devem ser declarados antes.

Rede: portais oficiais (open.toronto.ca, CKAN Toronto, ArcGIS TPS, openpolicing.stanford.edu,
data.cityofnewyork.us, Kaggle, Zenodo, data.gov) bloqueados pelo proxy (403). Ambos os arquivos são
espelhos no GitHub de dados oficiais publicados.

## 1. PRINCIPAL — toronto_arrests_strip_searches_2020_2021.csv

- Origem oficial: Toronto Police Service, "Police Race and Identity Based Data - Arrests and Strip
  Searches" (RBDC-ARR-TBL-001), City of Toronto Open Data (publicado 2022-12-02, "Will not be refreshed").
  ArcGIS: https://services.arcgis.com/S9th0jAJ7bqgIRjw/arcgis/rest/services/RBDC_ARR_TBL_001/FeatureServer/0/
  Portal: https://open.toronto.ca/dataset/police-race-and-identity-based-data-collection-arrests-strip-searches/
- Licença: Open Government Licence – Toronto (licença padrão do portal; não estava declarada no espelho).
- Espelho baixado: https://raw.githubusercontent.com/TheZarif/toronto-crisis-policing/HEAD/data/01-raw_data/arrests_strip_searches.csv
  (commit 5bc55245, 2026-10-09). Verificação cruzada: espelho independente
  https://raw.githubusercontent.com/cmax614/INF2178_Police_arrest_analysis/HEAD/Arrests_and_Strip_Searches_(RBDC-ARR-TBL-001)%20(1).csv
  é byte-idêntico exceto por um BOM UTF-8 de 3 bytes.
- Metadados oficiais (dicionário de colunas): toronto_metadata_open-data-toronto.yaml, de
  https://raw.githubusercontent.com/open-data-toronto/operations/HEAD/dags/datasets/files_to_datastore/archived_yamls/police-race-and-identity-based-data-collection-arrests-strip-searches.yaml
- Unidade: uma linha por pessoa-prisão. 65 276 linhas × 25 colunas; 60 003 EventID, 64 805 ArrestID
  (469 nulos), 37 347 PersonID (pessoas repetem-se → dependência entre linhas).
- Tempo: Arrest_Year ∈ {2020, 2021}; Arrest_Month = TRIMESTRE (Jan-Mar, Apr-June, July-Sept, Oct-Dec).
  Granularidade = trimestre, 8 trimestres (6 922–9 512 linhas cada). Sem dia/hora.
- Colunas (dtype; nulos):
  Arrest_Year int; Arrest_Month str; EventID int; ArrestID float(469 nulos); PersonID int;
  Perceived_Race str(4 nulos; Black, East/Southeast Asian, Indigenous, Latino, Middle-Eastern,
  South Asian, Unknown or Legacy, White) — percepção do policial;
  Sex str(F/M/U); Age_group__at_arrest_ str(24 nulos; 9 rótulos, com duplicatas de grafia
  "17 years and under"/"17 years and younger", "65 and older"/"65 years and older");
  Youth_at_arrest__under_18_years str(3 rótulos, mesma duplicação); ArrestLocDiv str(18 divisões, "XX"=fora/sem geocódigo);
  StripSearch int 0/1; Booked int 0/1; Occurrence_Category str(165 nulos; 31 categorias);
  Actions_at_arrest___{Concealed_i,Combative__,Resisted__d,Mental_inst,Assaulted_o,Cooperative} int 0/1;
  SearchReason_{CauseInjury,AssistEscape,PossessWeapons,PossessEvidence} float 0/1 (57 475 nulos = não revistado);
  ItemsFound float 0/1 (57 475 nulos); ObjectId int.
- Desfecho binário candidato: **Booked** (levado/registrado na delegacia em 24 h vs não — análogo a
  held vs released/summons). Alternativa: **StripSearch**. ATENÇÃO (metadados oficiais): por problema no
  template, há registros com StripSearch=1 e Booked=0; "presume-se que houve booking". SearchReason_* e
  ItemsFound só existem quando StripSearch=1 (pós-decisão; não usar como covariável). Actions_at_arrest_* são
  registros do policial no momento da prisão.

## 2. BACKUP — backup_rhode_island_traffic_stops_2005_2015.csv

- Origem: Stanford Open Policing Project, Rhode Island (state patrol) traffic stops, versão antiga.
  Licença: Open Data Commons Attribution License (ODC-By), conforme o README do espelho.
- Espelho baixado: https://raw.githubusercontent.com/justmarkham/pycon-2018-tutorial/master/police.csv
  (Kevin Markham, PyCon 2018). É um SUBCONJUNTO do arquivo RI completo (~500 mil paradas); critério de
  amostragem não documentado no espelho. county_name 100% nulo.
- 91 741 linhas × 15 colunas. Tempo: stop_date (DIA) + stop_time (HH:MM), 2005-01-02 a 2015-12-31
  (2005 parcial: 2 558; demais anos 7 561–10 970).
- Colunas: stop_date str; stop_time str; county_name (vazio); driver_gender str(F/M; 5 335 nulos);
  driver_age_raw float (ano de nascimento); driver_age float(5 621 nulos); driver_race str(Asian, Black,
  Hispanic, Other, White; 5 333 nulos); violation_raw str(12); violation str(6); search_conducted bool;
  search_type str(88 545 nulos); stop_outcome str(Arrest Driver, Arrest Passenger, Citation, N/D, No Action,
  Warning); is_arrested (bool c/ 5 333 nulos); stop_duration str(0-15 Min,16-30 Min,30+ Min + lixo "1","2");
  drugs_related_stop bool. As ~5 333 linhas sem raça/gênero/desfecho são as mesmas (registros vazios).
- Desfecho binário candidato: **is_arrested** (ou search_conducted; ou stop_outcome Citation vs Warning).
  search_type/drugs_related_stop são pós-decisão.
